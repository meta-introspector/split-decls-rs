// Generated macro for code_point_to_char (function)
macro_rules! Depcrate_wtf8_testscode_point_to_char {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_to_char"}
// Dependencies: {}
# [test] fn code_point_to_char () { fn c (value : u32) -> CodePoint { CodePoint :: from_u32 (value) . unwrap () } assert_eq ! (c (0x61) . to_char () , Some ('a')) ; assert_eq ! (c (0x1F4A9) . to_char () , Some ('💩')) ; assert_eq ! (c (0xD800) . to_char () , None) ; }
};
}
