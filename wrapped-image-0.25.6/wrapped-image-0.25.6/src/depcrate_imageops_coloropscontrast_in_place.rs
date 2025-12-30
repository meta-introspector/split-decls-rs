// Generated macro for contrast_in_place (function)
macro_rules! Depcrate_imageops_coloropscontrast_in_place {
() => {
// Module: crate::imageops::colorops
// Provides: {"contrast_in_place"}
// Dependencies: {}
# [doc = " Adjust the contrast of the supplied image in place."] # [doc = " ```contrast``` is the amount to adjust the contrast by."] # [doc = " Negative values decrease the contrast and positive values increase the contrast."] # [doc = ""] # [doc = " *[See also `contrast`.][contrast]*"] pub fn contrast_in_place < I > (image : & mut I , contrast : f32) where I : GenericImage , { let (width , height) = image . dimensions () ; let max = < I :: Pixel as Pixel > :: Subpixel :: DEFAULT_MAX_VALUE ; let max : f32 = NumCast :: from (max) . unwrap () ; let percent = ((100.0 + contrast) / 100.0) . powi (2) ; for y in 0 .. height { for x in 0 .. width { let f = image . get_pixel (x , y) . map (| b | { let c : f32 = NumCast :: from (b) . unwrap () ; let d = ((c / max - 0.5) * percent + 0.5) * max ; let e = clamp (d , 0.0 , max) ; NumCast :: from (e) . unwrap () }) ; image . put_pixel (x , y , f) ; } } }
};
}
