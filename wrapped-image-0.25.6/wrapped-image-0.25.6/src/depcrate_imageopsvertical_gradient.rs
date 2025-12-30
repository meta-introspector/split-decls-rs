// Generated macro for vertical_gradient (function)
macro_rules! Depcrate_imageopsvertical_gradient {
() => {
// Module: crate::imageops
// Provides: {"vertical_gradient"}
// Dependencies: {}
# [doc = " Fill the image with a linear vertical gradient"] # [doc = ""] # [doc = " This function assumes a linear color space."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use image::{Rgba, RgbaImage, Pixel};"] # [doc = ""] # [doc = " let mut img = RgbaImage::new(100, 100);"] # [doc = " let start = Rgba::from_slice(&[0, 128, 0, 0]);"] # [doc = " let end = Rgba::from_slice(&[255, 255, 255, 255]);"] # [doc = ""] # [doc = " image::imageops::vertical_gradient(&mut img, start, end);"] # [doc = " img.save(\"vertical_gradient.png\").unwrap();"] pub fn vertical_gradient < S , P , I > (img : & mut I , start : & P , stop : & P) where I : GenericImage < Pixel = P > , P : Pixel < Subpixel = S > + 'static , S : Primitive + Lerp + 'static , { for y in 0 .. img . height () { let pixel = start . map2 (stop , | a , b | { let y = < S :: Ratio as num_traits :: NumCast > :: from (y) . unwrap () ; let height = < S :: Ratio as num_traits :: NumCast > :: from (img . height () - 1) . unwrap () ; S :: lerp (a , b , y / height) }) ; for x in 0 .. img . width () { img . put_pixel (x , y , pixel) ; } } }
};
}
