// Generated macro for test_path_element (function)
macro_rules! Depcrate_element_basic_shapestest_path_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_path_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_path_element () { use crate :: prelude :: * ; let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_path (| c , s , path | { assert_eq ! (c , BLUE . to_rgba ()) ; assert_eq ! (s , 5) ; assert_eq ! (path , vec ! [(100 , 101) , (105 , 107) , (150 , 157)]) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_path_call , 1) ; assert_eq ! (b . draw_count , 1) ; }) ; }) ; da . draw (& PathElement :: new (vec ! [(100 , 101) , (105 , 107) , (150 , 157)] , Into :: < ShapeStyle > :: into (BLUE) . stroke_width (5) ,)) . expect ("Drawing Failure") ; }
};
}
