use clap::Parser;
use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage, ImageError, RgbImage};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let img = ImageReader::open(args.file.as_str())?.decode()?;

    let scaled = scale_image(&img);
    let grayscale = average_rgb_values(&scaled.as_rgb8().unwrap());
    let pixels = ascii_encode(&grayscale);

    display(pixels, scaled.width());
    println!();

    Ok(())
}

fn scale_image(img: &DynamicImage) -> DynamicImage {
    let new_width = img.width() / 10;
    let new_height = img.height() / 10;
    img.resize(new_width, new_height, FilterType::Triangle)
}

fn average_rgb_values(img: &RgbImage) -> Vec<u8> {
    let mut average_color: Vec<u8> = Vec::new();

    for x in 0..img.height() {
        for y in 0..img.width() {
            let pixel = img.get_pixel(y, x);
            average_color.push(((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8);
        }
    }

    average_color
}

fn ascii_encode(pixels: &Vec<u8>) -> Vec<char> {
    let mut ascii = Vec::new();
    for p in pixels {
        match p {
            253..=255 => ascii.push('@'),
            203..=252 => ascii.push('#'),
            152..=202 => ascii.push('$'),
            101..=151 => ascii.push('%'),
            51..=100 => ascii.push('|'),
            1..=50 => ascii.push(';'),
            0 => ascii.push(':'),
        }
    }
    ascii
}

fn display(pixels: Vec<char>, width: u32) {
    println!();
    for i in 0..pixels.len() {
        print!("{}", pixels[i]);
        if i as u32 % width == 0 {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    fn average_all_black_image() {
        let mut img = RgbImage::new(4, 4);

        for x in 0..4 {
            for y in 0..4 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }

        assert_eq!(
            average_rgb_values(&img),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn average_all_white_image() {
        let mut img = RgbImage::new(4, 4);

        for x in 0..4 {
            for y in 0..4 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }

        assert_eq!(
            average_rgb_values(&img),
            vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
    }
}
