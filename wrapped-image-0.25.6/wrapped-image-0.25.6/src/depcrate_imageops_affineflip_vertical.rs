// Generated macro for flip_vertical (function)
macro_rules! Depcrate_imageops_affineflip_vertical {
() => {
// Module: crate::imageops::affine
// Provides: {"flip_vertical"}
// Dependencies: {}
# [doc = " Flip an image vertically"] pub fn flip_vertical < I : GenericImageView > (image : & I ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (width , height) ; let _ = flip_vertical_in (image , & mut out) ; out }
};
}
