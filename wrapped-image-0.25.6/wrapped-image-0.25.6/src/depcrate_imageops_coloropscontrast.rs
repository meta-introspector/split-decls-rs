// Generated macro for contrast (function)
macro_rules! Depcrate_imageops_coloropscontrast {
() => {
// Module: crate::imageops::colorops
// Provides: {"contrast"}
// Dependencies: {}
# [doc = " Adjust the contrast of the supplied image."] # [doc = " ```contrast``` is the amount to adjust the contrast by."] # [doc = " Negative values decrease the contrast and positive values increase the contrast."] # [doc = ""] # [doc = " *[See also `contrast_in_place`.][contrast_in_place]*"] pub fn contrast < I , P , S > (image : & I , contrast : f32) -> ImageBuffer < P , Vec < S > > where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > + 'static , S : Primitive + 'static , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (width , height) ; let max = S :: DEFAULT_MAX_VALUE ; let max : f32 = NumCast :: from (max) . unwrap () ; let percent = ((100.0 + contrast) / 100.0) . powi (2) ; for (x , y , pixel) in image . pixels () { let f = pixel . map (| b | { let c : f32 = NumCast :: from (b) . unwrap () ; let d = ((c / max - 0.5) * percent + 0.5) * max ; let e = clamp (d , 0.0 , max) ; NumCast :: from (e) . unwrap () }) ; out . put_pixel (x , y , f) ; } out }
};
}
