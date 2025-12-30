// Generated macro for flip_vertical_in_place (function)
macro_rules! Depcrate_imageops_affineflip_vertical_in_place {
() => {
// Module: crate::imageops::affine
// Provides: {"flip_vertical_in_place"}
// Dependencies: {}
# [doc = " Flip an image vertically in place."] pub fn flip_vertical_in_place < I : GenericImage > (image : & mut I) { let (width , height) = image . dimensions () ; for y in 0 .. height / 2 { for x in 0 .. width { let y2 = height - y - 1 ; let p2 = image . get_pixel (x , y2) ; let p = image . get_pixel (x , y) ; image . put_pixel (x , y2 , p) ; image . put_pixel (x , y , p2) ; } } }
};
}
