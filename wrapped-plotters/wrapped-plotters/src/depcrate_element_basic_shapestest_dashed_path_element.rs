// Generated macro for test_dashed_path_element (function)
macro_rules! Depcrate_element_basic_shapestest_dashed_path_element {
() => {
// Module: crate::element::basic_shapes
// Provides: {"test_dashed_path_element"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_dashed_path_element () { use crate :: prelude :: * ; let check_list = std :: cell :: RefCell :: new (vec ! [vec ! [(100 , 100) , (100 , 103) , (100 , 105)] , vec ! [(100 , 107) , (100 , 112)] , vec ! [(100 , 114) , (100 , 119)] , vec ! [(100 , 119) , (100 , 120)] ,]) ; let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_path (move | c , s , path | { assert_eq ! (c , BLUE . to_rgba ()) ; assert_eq ! (s , 7) ; assert_eq ! (path , check_list . borrow_mut () . remove (0)) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_path_call , 3) ; assert_eq ! (b . draw_count , 3) ; }) ; }) ; da . draw (& DashedPathElement :: new (vec ! [(100 , 100) , (100 , 103) , (100 , 120)] , 5. , 2. , BLUE . stroke_width (7) ,)) . expect ("Drawing Failure") ; }
};
}
