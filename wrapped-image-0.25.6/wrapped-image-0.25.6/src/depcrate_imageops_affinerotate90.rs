// Generated macro for rotate90 (function)
macro_rules! Depcrate_imageops_affinerotate90 {
() => {
// Module: crate::imageops::affine
// Provides: {"rotate90"}
// Dependencies: {}
# [doc = " Rotate an image 90 degrees clockwise."] pub fn rotate90 < I : GenericImageView > (image : & I ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (height , width) ; let _ = rotate90_in (image , & mut out) ; out }
};
}
