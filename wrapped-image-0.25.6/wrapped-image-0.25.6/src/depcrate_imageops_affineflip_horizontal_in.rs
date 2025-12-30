// Generated macro for flip_horizontal_in (function)
macro_rules! Depcrate_imageops_affineflip_horizontal_in {
() => {
// Module: crate::imageops::affine
// Provides: {"flip_horizontal_in"}
// Dependencies: {}
# [doc = " Flip an image horizontally and put the result into the destination [`ImageBuffer`]."] pub fn flip_horizontal_in < I , Container > (image : & I , destination : & mut ImageBuffer < I :: Pixel , Container > ,) -> crate :: ImageResult < () > where I : GenericImageView , I :: Pixel : 'static , Container : std :: ops :: DerefMut < Target = [< I :: Pixel as Pixel > :: Subpixel] > , { let ((w0 , h0) , (w1 , h1)) = (image . dimensions () , destination . dimensions ()) ; if w0 != w1 || h0 != h1 { return Err (ImageError :: Parameter (ParameterError :: from_kind (ParameterErrorKind :: DimensionMismatch ,))) ; } for y in 0 .. h0 { for x in 0 .. w0 { let p = image . get_pixel (x , y) ; destination . put_pixel (w0 - x - 1 , y , p) ; } } Ok (()) }
};
}
