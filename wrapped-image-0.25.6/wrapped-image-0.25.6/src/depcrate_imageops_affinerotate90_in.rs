// Generated macro for rotate90_in (function)
macro_rules! Depcrate_imageops_affinerotate90_in {
() => {
// Module: crate::imageops::affine
// Provides: {"rotate90_in"}
// Dependencies: {}
# [doc = " Rotate an image 90 degrees clockwise and put the result into the destination [`ImageBuffer`]."] pub fn rotate90_in < I , Container > (image : & I , destination : & mut ImageBuffer < I :: Pixel , Container > ,) -> crate :: ImageResult < () > where I : GenericImageView , I :: Pixel : 'static , Container : std :: ops :: DerefMut < Target = [< I :: Pixel as Pixel > :: Subpixel] > , { let ((w0 , h0) , (w1 , h1)) = (image . dimensions () , destination . dimensions ()) ; if w0 != h1 || h0 != w1 { return Err (ImageError :: Parameter (ParameterError :: from_kind (ParameterErrorKind :: DimensionMismatch ,))) ; } for y in 0 .. h0 { for x in 0 .. w0 { let p = image . get_pixel (x , y) ; destination . put_pixel (h0 - y - 1 , x , p) ; } } Ok (()) }
};
}
