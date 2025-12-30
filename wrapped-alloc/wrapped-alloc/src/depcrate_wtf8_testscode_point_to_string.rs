// Generated macro for code_point_to_string (function)
macro_rules! Depcrate_wtf8_testscode_point_to_string {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_to_string"}
// Dependencies: {}
# [test] fn code_point_to_string () { assert_eq ! (format ! ("{:?}" , CodePoint :: from_char ('a')) , "U+0061") ; assert_eq ! (format ! ("{:?}" , CodePoint :: from_char ('💩')) , "U+1F4A9") ; }
};
}
