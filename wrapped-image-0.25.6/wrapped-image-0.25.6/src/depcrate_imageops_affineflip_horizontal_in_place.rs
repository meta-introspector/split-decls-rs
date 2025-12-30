// Generated macro for flip_horizontal_in_place (function)
macro_rules! Depcrate_imageops_affineflip_horizontal_in_place {
() => {
// Module: crate::imageops::affine
// Provides: {"flip_horizontal_in_place"}
// Dependencies: {}
# [doc = " Flip an image horizontally in place."] pub fn flip_horizontal_in_place < I : GenericImage > (image : & mut I) { let (width , height) = image . dimensions () ; for y in 0 .. height { for x in 0 .. width / 2 { let x2 = width - x - 1 ; let p2 = image . get_pixel (x2 , y) ; let p = image . get_pixel (x , y) ; image . put_pixel (x2 , y , p) ; image . put_pixel (x , y , p2) ; } } }
};
}
