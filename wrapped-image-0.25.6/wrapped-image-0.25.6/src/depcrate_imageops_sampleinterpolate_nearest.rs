// Generated macro for interpolate_nearest (function)
macro_rules! Depcrate_imageops_sampleinterpolate_nearest {
() => {
// Module: crate::imageops::sample
// Provides: {"interpolate_nearest"}
// Dependencies: {}
# [doc = " Sample from an image using coordinates in [0, w-1] and [0, h-1], taking the"] # [doc = " nearest pixel."] # [doc = ""] # [doc = " Coordinates outside the image bounds will return `None`, however the"] # [doc = " behavior for points within half a pixel of the image bounds may change in"] # [doc = " the future."] pub fn interpolate_nearest < P : Pixel > (img : & impl GenericImageView < Pixel = P > , x : f32 , y : f32 ,) -> Option < P > { let (w , h) = img . dimensions () ; if w == 0 || h == 0 { return None ; } if ! (0.0 ..= ((w - 1) as f32)) . contains (& x) { return None ; } if ! (0.0 ..= ((h - 1) as f32)) . contains (& y) { return None ; } Some (img . get_pixel (x . round () as u32 , y . round () as u32)) }
};
}
