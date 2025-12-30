// Generated macro for rotate180 (function)
macro_rules! Depcrate_imageops_affinerotate180 {
() => {
// Module: crate::imageops::affine
// Provides: {"rotate180"}
// Dependencies: {}
# [doc = " Rotate an image 180 degrees clockwise."] pub fn rotate180 < I : GenericImageView > (image : & I ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (width , height) ; let _ = rotate180_in (image , & mut out) ; out }
};
}
