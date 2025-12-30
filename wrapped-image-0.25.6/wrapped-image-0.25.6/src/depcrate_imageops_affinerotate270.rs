// Generated macro for rotate270 (function)
macro_rules! Depcrate_imageops_affinerotate270 {
() => {
// Module: crate::imageops::affine
// Provides: {"rotate270"}
// Dependencies: {}
# [doc = " Rotate an image 270 degrees clockwise."] pub fn rotate270 < I : GenericImageView > (image : & I ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (height , width) ; let _ = rotate270_in (image , & mut out) ; out }
};
}
