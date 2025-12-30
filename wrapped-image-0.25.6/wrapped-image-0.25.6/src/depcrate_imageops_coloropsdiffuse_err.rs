// Generated macro for diffuse_err (function)
macro_rules! Depcrate_imageops_coloropsdiffuse_err {
() => {
// Module: crate::imageops::colorops
// Provides: {"diffuse_err"}
// Dependencies: {}
# [doc = " Floyd-Steinberg error diffusion"] fn diffuse_err < P : Pixel < Subpixel = u8 > > (pixel : & mut P , error : [i16 ; 3] , factor : i16) { for (e , c) in error . iter () . zip (pixel . channels_mut () . iter_mut ()) { * c = match < i16 as From < _ > > :: from (* c) + e * factor / 16 { val if val < 0 => 0 , val if val > 0xFF => 0xFF , val => val as u8 , } } }
};
}
