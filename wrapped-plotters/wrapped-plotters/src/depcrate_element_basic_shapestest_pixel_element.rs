// Generated macro for test_pixel_element (function)
macro_rules! Depcrate_element_basic_shapestest_pixel_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_pixel_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_pixel_element () { use crate :: prelude :: * ; let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_pixel (| c , (x , y) | { assert_eq ! (x , 150) ; assert_eq ! (y , 152) ; assert_eq ! (c , RED . to_rgba ()) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_pixel_call , 1) ; assert_eq ! (b . draw_count , 1) ; }) ; }) ; da . draw (& Pixel :: new ((150 , 152) , RED)) . expect ("Drawing Failure") ; }
};
}
