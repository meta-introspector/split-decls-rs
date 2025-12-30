// Generated macro for test_circle_element (function)
macro_rules! Depcrate_element_basic_shapestest_circle_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_circle_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_circle_element () { use crate :: prelude :: * ; let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_circle (| c , _ , f , s , r | { assert_eq ! (c , BLUE . to_rgba ()) ; assert ! (! f) ; assert_eq ! (s , (150 , 151)) ; assert_eq ! (r , 20) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_circle_call , 1) ; assert_eq ! (b . draw_count , 1) ; }) ; }) ; da . draw (& Circle :: new ((150 , 151) , 20 , BLUE)) . expect ("Drawing Failure") ; }
};
}
