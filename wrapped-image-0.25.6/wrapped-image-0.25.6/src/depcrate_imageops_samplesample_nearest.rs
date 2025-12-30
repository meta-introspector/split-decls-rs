// Generated macro for sample_nearest (function)
macro_rules! Depcrate_imageops_samplesample_nearest {
() => {
// Module: crate::imageops::sample
// Provides: {"sample_nearest"}
// Dependencies: {}
# [doc = " Sample from an image using coordinates in [0, 1], taking the nearest coordinate."] pub fn sample_nearest < P : Pixel > (img : & impl GenericImageView < Pixel = P > , u : f32 , v : f32 ,) -> Option < P > { if ! [u , v] . iter () . all (| c | (0.0 ..= 1.0) . contains (c)) { return None ; } let (w , h) = img . dimensions () ; let ui = w as f32 * u - 0.5 ; let ui = ui . max (0.) . min ((w . saturating_sub (1)) as f32) ; let vi = h as f32 * v - 0.5 ; let vi = vi . max (0.) . min ((h . saturating_sub (1)) as f32) ; interpolate_nearest (img , ui , vi) }
};
}
