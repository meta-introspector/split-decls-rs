// Generated macro for brighten (function)
macro_rules! Depcrate_imageops_coloropsbrighten {
() => {
// Module: crate::imageops::colorops
// Provides: {"brighten"}
// Dependencies: {}
# [doc = " Brighten the supplied image."] # [doc = " ```value``` is the amount to brighten each pixel by."] # [doc = " Negative values decrease the brightness and positive values increase it."] # [doc = ""] # [doc = " *[See also `brighten_in_place`.][brighten_in_place]*"] pub fn brighten < I , P , S > (image : & I , value : i32) -> ImageBuffer < P , Vec < S > > where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > + 'static , S : Primitive + 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (width , height) ; let max = S :: DEFAULT_MAX_VALUE ; let max : i32 = NumCast :: from (max) . unwrap () ; for (x , y , pixel) in image . pixels () { let e = pixel . map_with_alpha (| b | { let c : i32 = NumCast :: from (b) . unwrap () ; let d = clamp (c + value , 0 , max) ; NumCast :: from (d) . unwrap () } , | alpha | alpha ,) ; out . put_pixel (x , y , e) ; } out }
};
}
