// Generated macro for unsharpen (function)
macro_rules! Depcrate_imageops_sampleunsharpen {
() => {
// Module: crate::imageops::sample
// Provides: {"unsharpen"}
// Dependencies: {}
# [doc = " Performs an unsharpen mask on the supplied image."] # [doc = " ```sigma``` is the amount to blur the image by."] # [doc = " ```threshold``` is the threshold for minimal brightness change that will be sharpened."] # [doc = ""] # [doc = " See <https://en.wikipedia.org/wiki/Unsharp_masking#Digital_unsharp_masking>"] pub fn unsharpen < I , P , S > (image : & I , sigma : f32 , threshold : i32) -> ImageBuffer < P , Vec < S > > where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > + 'static , S : Primitive + 'static , { let mut tmp = blur (image , sigma) ; let max = S :: DEFAULT_MAX_VALUE ; let max : i32 = NumCast :: from (max) . unwrap () ; let (width , height) = image . dimensions () ; for y in 0 .. height { for x in 0 .. width { let a = image . get_pixel (x , y) ; let b = tmp . get_pixel_mut (x , y) ; let p = a . map2 (b , | c , d | { let ic : i32 = NumCast :: from (c) . unwrap () ; let id : i32 = NumCast :: from (d) . unwrap () ; let diff = ic - id ; if diff . abs () > threshold { let e = clamp (ic + diff , 0 , max) ; NumCast :: from (e) . unwrap () } else { c } }) ; * b = p ; } } tmp }
};
}
