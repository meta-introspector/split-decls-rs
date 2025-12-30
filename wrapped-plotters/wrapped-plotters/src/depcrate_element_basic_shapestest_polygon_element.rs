// Generated macro for test_polygon_element (function)
macro_rules! Depcrate_element_basic_shapestest_polygon_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_polygon_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_polygon_element () { use crate :: prelude :: * ; let points = vec ! [(100 , 100) , (50 , 500) , (300 , 400) , (200 , 300) , (550 , 200)] ; let expected_points = points . clone () ; let da = crate :: create_mocked_drawing_area (800 , 800 , | m | { m . check_fill_polygon (move | c , p | { assert_eq ! (c , BLUE . to_rgba ()) ; assert_eq ! (expected_points . len () , p . len ()) ; assert_eq ! (expected_points , p) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_fill_polygon_call , 1) ; assert_eq ! (b . draw_count , 1) ; }) ; }) ; da . draw (& Polygon :: new (points . clone () , BLUE)) . expect ("Drawing Failure") ; }
};
}
