// Generated macro for rotate270_in (function)
macro_rules! Depcrate_imageops_affinerotate270_in {
() => {
// Module: crate::imageops::affine
// Provides: {"rotate270_in"}
// Dependencies: {}
# [doc = " Rotate an image 270 degrees clockwise and put the result into the destination [`ImageBuffer`]."] pub fn rotate270_in < I , Container > (image : & I , destination : & mut ImageBuffer < I :: Pixel , Container > ,) -> crate :: ImageResult < () > where I : GenericImageView , I :: Pixel : 'static , Container : std :: ops :: DerefMut < Target = [< I :: Pixel as Pixel > :: Subpixel] > , { let ((w0 , h0) , (w1 , h1)) = (image . dimensions () , destination . dimensions ()) ; if w0 != h1 || h0 != w1 { return Err (ImageError :: Parameter (ParameterError :: from_kind (ParameterErrorKind :: DimensionMismatch ,))) ; } for y in 0 .. h0 { for x in 0 .. w0 { let p = image . get_pixel (x , y) ; destination . put_pixel (y , w0 - x - 1 , p) ; } } Ok (()) }
};
}
