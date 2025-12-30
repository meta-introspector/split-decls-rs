// Generated macro for test_preserve_stroke_width (function)
macro_rules! Depcrate_element_errorbartest_preserve_stroke_width {
() => {
// Module: crate::element::errorbar
// Provides: {"test_preserve_stroke_width"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_preserve_stroke_width () { let v = ErrorBar :: new_vertical (100 , 20 , 50 , 70 , WHITE . filled () . stroke_width (5) , 3) ; let h = ErrorBar :: new_horizontal (100 , 20 , 50 , 70 , WHITE . filled () . stroke_width (5) , 3) ; use crate :: prelude :: * ; let da = crate :: create_mocked_drawing_area (300 , 300 , | m | { m . check_draw_line (| _ , w , _ , _ | { assert_eq ! (w , 5) ; }) ; }) ; da . draw (& h) . expect ("Drawing Failure") ; da . draw (& v) . expect ("Drawing Failure") ; }
};
}
