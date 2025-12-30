// Generated macro for code_point_from_char (function)
macro_rules! Depcrate_wtf8_testscode_point_from_char {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_from_char"}
// Dependencies: {}
# [test] fn code_point_from_char () { assert_eq ! (CodePoint :: from_char ('a') . to_u32 () , 0x61) ; assert_eq ! (CodePoint :: from_char ('💩') . to_u32 () , 0x1F4A9) ; }
};
}
