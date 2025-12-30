// Generated macro for brighten_in_place (function)
macro_rules! Depcrate_imageops_coloropsbrighten_in_place {
() => {
// Module: crate::imageops::colorops
// Provides: {"brighten_in_place"}
// Dependencies: {}
# [doc = " Brighten the supplied image in place."] # [doc = " ```value``` is the amount to brighten each pixel by."] # [doc = " Negative values decrease the brightness and positive values increase it."] # [doc = ""] # [doc = " *[See also `brighten`.][brighten]*"] pub fn brighten_in_place < I > (image : & mut I , value : i32) where I : GenericImage , { let (width , height) = image . dimensions () ; let max = < I :: Pixel as Pixel > :: Subpixel :: DEFAULT_MAX_VALUE ; let max : i32 = NumCast :: from (max) . unwrap () ; for y in 0 .. height { for x in 0 .. width { let e = image . get_pixel (x , y) . map_with_alpha (| b | { let c : i32 = NumCast :: from (b) . unwrap () ; let d = clamp (c + value , 0 , max) ; NumCast :: from (d) . unwrap () } , | alpha | alpha ,) ; image . put_pixel (x , y , e) ; } } }
};
}
