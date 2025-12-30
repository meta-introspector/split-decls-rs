// Generated macro for replace (function)
macro_rules! Depcrate_imageopsreplace {
() => {
// Module: crate::imageops
// Provides: {"replace"}
// Dependencies: {}
# [doc = " Replace the contents of an image at a given coordinate (x, y)"] pub fn replace < I , J > (bottom : & mut I , top : & J , x : i64 , y : i64) where I : GenericImage , J : GenericImageView < Pixel = I :: Pixel > , { let bottom_dims = bottom . dimensions () ; let top_dims = top . dimensions () ; let (origin_bottom_x , origin_bottom_y , origin_top_x , origin_top_y , range_width , range_height) = overlay_bounds_ext (bottom_dims , top_dims , x , y) ; for y in 0 .. range_height { for x in 0 .. range_width { let p = top . get_pixel (origin_top_x + x , origin_top_y + y) ; bottom . put_pixel (origin_bottom_x + x , origin_bottom_y + y , p) ; } } }
};
}
