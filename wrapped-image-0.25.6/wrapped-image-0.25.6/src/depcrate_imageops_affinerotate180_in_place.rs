// Generated macro for rotate180_in_place (function)
macro_rules! Depcrate_imageops_affinerotate180_in_place {
() => {
// Module: crate::imageops::affine
// Provides: {"rotate180_in_place"}
// Dependencies: {}
# [doc = " Rotate an image 180 degrees clockwise in place."] pub fn rotate180_in_place < I : GenericImage > (image : & mut I) { let (width , height) = image . dimensions () ; for y in 0 .. height / 2 { for x in 0 .. width { let p = image . get_pixel (x , y) ; let x2 = width - x - 1 ; let y2 = height - y - 1 ; let p2 = image . get_pixel (x2 , y2) ; image . put_pixel (x , y , p2) ; image . put_pixel (x2 , y2 , p) ; } } if height % 2 != 0 { let middle = height / 2 ; for x in 0 .. width / 2 { let p = image . get_pixel (x , middle) ; let x2 = width - x - 1 ; let p2 = image . get_pixel (x2 , middle) ; image . put_pixel (x , middle , p2) ; image . put_pixel (x2 , middle , p) ; } } }
};
}
