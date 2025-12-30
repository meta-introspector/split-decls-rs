// Generated macro for fast_blur (function)
macro_rules! Depcrate_imageops_fast_blurfast_blur {
() => {
// Module: crate::imageops::fast_blur
// Provides: {"fast_blur"}
// Dependencies: {}
# [doc = " Approximation of Gaussian blur after"] # [doc = " Kovesi, P.:  Fast Almost-Gaussian Filtering The Australian Pattern"] # [doc = " Recognition Society Conference: DICTA 2010. December 2010. Sydney."] # [doc = " This method assumes alpha pre-multiplication for images that contain non-constant alpha."] # [must_use] pub fn fast_blur < P : Pixel > (image_buffer : & ImageBuffer < P , Vec < P :: Subpixel > > , sigma : f32 ,) -> ImageBuffer < P , Vec < P :: Subpixel > > { let (width , height) = image_buffer . dimensions () ; if width == 0 || height == 0 { return image_buffer . clone () ; } let mut samples = image_buffer . as_flat_samples () . samples . to_vec () ; let num_passes = 3 ; let boxes = boxes_for_gauss (sigma , num_passes) ; for radius in boxes . iter () . take (num_passes) { let horizontally_blurred_transposed = horizontal_fast_blur_half :: < P :: Subpixel > (& samples , width as usize , height as usize , (* radius - 1) / 2 , P :: CHANNEL_COUNT as usize ,) ; samples = horizontal_fast_blur_half :: < P :: Subpixel > (& horizontally_blurred_transposed , height as usize , width as usize , (* radius - 1) / 2 , P :: CHANNEL_COUNT as usize ,) ; } ImageBuffer :: from_raw (width , height , samples) . unwrap () }
};
}
