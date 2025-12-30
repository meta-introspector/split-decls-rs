// Generated macro for flip_horizontal (function)
macro_rules! Depcrate_imageops_affineflip_horizontal {
() => {
// Module: crate::imageops::affine
// Provides: {"flip_horizontal"}
// Dependencies: {}
# [doc = " Flip an image horizontally"] pub fn flip_horizontal < I : GenericImageView > (image : & I ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (width , height) ; let _ = flip_horizontal_in (image , & mut out) ; out }
};
}
