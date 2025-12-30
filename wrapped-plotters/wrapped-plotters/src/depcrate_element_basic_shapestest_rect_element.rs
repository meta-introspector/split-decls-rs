// Generated macro for test_rect_element (function)
macro_rules! Depcrate_element_basic_shapestest_rect_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_rect_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_rect_element () { use crate :: prelude :: * ; { let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_rect (| c , s , f , u , d | { assert_eq ! (c , BLUE . to_rgba ()) ; assert ! (! f) ; assert_eq ! (s , 5) ; assert_eq ! ([u , d] , [(100 , 101) , (105 , 107)]) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_rect_call , 1) ; assert_eq ! (b . draw_count , 1) ; }) ; }) ; da . draw (& Rectangle :: new ([(100 , 101) , (105 , 107)] , Color :: stroke_width (& BLUE , 5) ,)) . expect ("Drawing Failure") ; } { let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_rect (| c , _ , f , u , d | { assert_eq ! (c , BLUE . to_rgba ()) ; assert ! (f) ; assert_eq ! ([u , d] , [(100 , 101) , (105 , 107)]) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_rect_call , 1) ; assert_eq ! (b . draw_count , 1) ; }) ; }) ; da . draw (& Rectangle :: new ([(100 , 101) , (105 , 107)] , BLUE . filled ())) . expect ("Drawing Failure") ; } }
};
}
