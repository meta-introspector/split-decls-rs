// Generated macro for create_bitmap_context_test (function)
macro_rules! Depcrate_contextcreate_bitmap_context_test {
() => {
// Module: crate::context
// Provides: {"create_bitmap_context_test"}
// Dependencies: {}
# [test] fn create_bitmap_context_test () { use crate :: geometry :: * ; let cs = CGColorSpace :: create_device_rgb () ; let ctx = CGContext :: create_bitmap_context (None , 16 , 8 , 8 , 0 , & cs , crate :: base :: kCGImageAlphaPremultipliedLast ,) ; ctx . set_rgb_fill_color (1. , 0. , 1. , 1.) ; ctx . set_miter_limit (4.) ; ctx . fill_rect (CGRect :: new (& CGPoint :: new (0. , 0.) , & CGSize :: new (8. , 8.))) ; let img = ctx . create_image () . unwrap () ; assert_eq ! (16 , img . width ()) ; assert_eq ! (8 , img . height ()) ; assert_eq ! (8 , img . bits_per_component ()) ; assert_eq ! (32 , img . bits_per_pixel ()) ; let data = img . data () ; assert_eq ! (255 , data . bytes () [0]) ; assert_eq ! (0 , data . bytes () [1]) ; assert_eq ! (255 , data . bytes () [2]) ; assert_eq ! (255 , data . bytes () [3]) ; }
};
}
