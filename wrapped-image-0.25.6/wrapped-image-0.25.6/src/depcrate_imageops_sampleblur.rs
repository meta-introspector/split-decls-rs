// Generated macro for blur (function)
macro_rules! Depcrate_imageops_sampleblur {
() => {
// Module: crate::imageops::sample
// Provides: {"blur"}
// Dependencies: {}
# [doc = " Performs a Gaussian blur on the supplied image."] # [doc = " ```sigma``` is a measure of how much to blur by."] # [doc = " Use [`crate::imageops::fast_blur()`] for a faster but less"] # [doc = " accurate version."] # [doc = " This method assumes alpha pre-multiplication for images that contain non-constant alpha."] pub fn blur < I : GenericImageView > (image : & I , sigma : f32 ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , { let sigma = if sigma <= 0.0 { 1.0 } else { sigma } ; let mut method = Filter { kernel : Box :: new (| x | gaussian (x , sigma)) , support : 2.0 * sigma , } ; let (width , height) = image . dimensions () ; let is_empty = width == 0 || height == 0 ; if is_empty { return ImageBuffer :: new (width , height) ; } let tmp : Rgba32FImage = vertical_sample (image , height , & mut method) ; horizontal_sample (& tmp , width , & mut method) }
};
}
