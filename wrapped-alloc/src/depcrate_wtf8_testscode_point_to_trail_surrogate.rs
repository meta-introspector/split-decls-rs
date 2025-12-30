// Generated macro for code_point_to_trail_surrogate (function)
macro_rules! Depcrate_wtf8_testscode_point_to_trail_surrogate {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_to_trail_surrogate"}
// Dependencies: {}
# [test] fn code_point_to_trail_surrogate () { fn c (value : u32) -> CodePoint { CodePoint :: from_u32 (value) . unwrap () } assert_eq ! (c (0) . to_trail_surrogate () , None) ; assert_eq ! (c (0xE9) . to_trail_surrogate () , None) ; assert_eq ! (c (0xD800) . to_trail_surrogate () , None) ; assert_eq ! (c (0xDBFF) . to_trail_surrogate () , None) ; assert_eq ! (c (0xDC00) . to_trail_surrogate () , Some (0xDC00)) ; assert_eq ! (c (0xDFFF) . to_trail_surrogate () , Some (0xDFFF)) ; assert_eq ! (c (0x1F4A9) . to_trail_surrogate () , None) ; assert_eq ! (c (0x10FFFF) . to_trail_surrogate () , None) ; }
};
}
