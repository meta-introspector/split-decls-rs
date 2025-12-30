// Generated macro for sample_bilinear (function)
macro_rules! Depcrate_imageops_samplesample_bilinear {
() => {
// Module: crate::imageops::sample
// Provides: {"sample_bilinear"}
// Dependencies: {}
# [doc = " Linearly sample from an image using coordinates in [0, 1]."] pub fn sample_bilinear < P : Pixel > (img : & impl GenericImageView < Pixel = P > , u : f32 , v : f32 ,) -> Option < P > { if ! [u , v] . iter () . all (| c | (0.0 ..= 1.0) . contains (c)) { return None ; } let (w , h) = img . dimensions () ; if w == 0 || h == 0 { return None ; } let ui = w as f32 * u - 0.5 ; let vi = h as f32 * v - 0.5 ; interpolate_bilinear (img , ui . max (0.) . min ((w - 1) as f32) , vi . max (0.) . min ((h - 1) as f32) ,) }
};
}
