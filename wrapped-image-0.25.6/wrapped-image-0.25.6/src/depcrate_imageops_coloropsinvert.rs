// Generated macro for invert (function)
macro_rules! Depcrate_imageops_coloropsinvert {
() => {
// Module: crate::imageops::colorops
// Provides: {"invert"}
// Dependencies: {}
# [doc = " Invert each pixel within the supplied image."] # [doc = " This function operates in place."] pub fn invert < I : GenericImage > (image : & mut I) { let (width , height) = image . dimensions () ; for y in 0 .. height { for x in 0 .. width { let mut p = image . get_pixel (x , y) ; p . invert () ; image . put_pixel (x , y , p) ; } } }
};
}
