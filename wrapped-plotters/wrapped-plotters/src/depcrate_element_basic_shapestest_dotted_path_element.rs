// Generated macro for test_dotted_path_element (function)
macro_rules! Depcrate_element_basic_shapestest_dotted_path_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_dotted_path_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_dotted_path_element () { use crate :: prelude :: * ; let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . drop_check (| b | { assert_eq ! (b . num_draw_path_call , 0) ; assert_eq ! (b . draw_count , 7) ; }) ; }) ; da . draw (& DottedPathElement :: new (vec ! [(100 , 100) , (105 , 105) , (150 , 150)] , 5 , 10 , | c | Circle :: new (c , 5 , Into :: < ShapeStyle > :: into (RED) . filled ()) ,)) . expect ("Drawing Failure") ; }
};
}
