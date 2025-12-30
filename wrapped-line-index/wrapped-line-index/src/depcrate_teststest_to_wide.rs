// Generated macro for test_to_wide (function)
macro_rules! Depcrate_teststest_to_wide {
() => {
// Module: crate::tests
// Provides: {"test_to_wide"}
// Dependencies: {}
# [test] fn test_to_wide () { let text = "\n\n\n\n\n宽3456" ; assert_eq ! (& text [5 .. 8] , "宽") ; assert_eq ! (& text [11 .. 12] , "6") ; let line_index = LineIndex :: new (text) ; let before_6 = TextSize :: from (11) ; let line_col = line_index . try_line_col (before_6) ; assert_eq ! (line_col , Some (LineCol { line : 5 , col : 6 })) ; let wide_line_col = line_index . to_wide (WideEncoding :: Utf16 , line_col . unwrap ()) ; assert_eq ! (wide_line_col , Some (WideLineCol { line : 5 , col : 4 })) ; }
};
}
