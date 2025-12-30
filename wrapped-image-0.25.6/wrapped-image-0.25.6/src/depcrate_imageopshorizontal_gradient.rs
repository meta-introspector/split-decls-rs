// Generated macro for horizontal_gradient (function)
macro_rules! Depcrate_imageopshorizontal_gradient {
() => {
// Module: crate::imageops
// Provides: {"horizontal_gradient"}
// Dependencies: {}
# [doc = " Fill the image with a linear horizontal gradient"] # [doc = ""] # [doc = " This function assumes a linear color space."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use image::{Rgba, RgbaImage, Pixel};"] # [doc = ""] # [doc = " let mut img = RgbaImage::new(100, 100);"] # [doc = " let start = Rgba::from_slice(&[0, 128, 0, 0]);"] # [doc = " let end = Rgba::from_slice(&[255, 255, 255, 255]);"] # [doc = ""] # [doc = " image::imageops::horizontal_gradient(&mut img, start, end);"] # [doc = " img.save(\"horizontal_gradient.png\").unwrap();"] pub fn horizontal_gradient < S , P , I > (img : & mut I , start : & P , stop : & P) where I : GenericImage < Pixel = P > , P : Pixel < Subpixel = S > + 'static , S : Primitive + Lerp + 'static , { for x in 0 .. img . width () { let pixel = start . map2 (stop , | a , b | { let x = < S :: Ratio as num_traits :: NumCast > :: from (x) . unwrap () ; let width = < S :: Ratio as num_traits :: NumCast > :: from (img . width () - 1) . unwrap () ; S :: lerp (a , b , x / width) }) ; for y in 0 .. img . height () { img . put_pixel (x , y , pixel) ; } } }
};
}
