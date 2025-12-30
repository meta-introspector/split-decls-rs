// Generated macro for test_try_line_col (function)
macro_rules! Depcrate_teststest_try_line_col {
() => {
// Module: crate::tests
// Provides: {"test_try_line_col"}
// Dependencies: {}
# [test] fn test_try_line_col () { let text = "\n\n\n\n\n宽3456" ; assert_eq ! (& text [5 .. 8] , "宽") ; assert_eq ! (& text [11 .. 12] , "6") ; let line_index = LineIndex :: new (text) ; let before_6 = TextSize :: from (11) ; let line_col = line_index . try_line_col (before_6) ; assert_eq ! (line_col , Some (LineCol { line : 5 , col : 6 })) ; }
};
}
