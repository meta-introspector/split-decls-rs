// Generated macro for code_point_to_u32 (function)
macro_rules! Depcrate_wtf8_testscode_point_to_u32 {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_to_u32"}
// Dependencies: {}
# [test] fn code_point_to_u32 () { fn c (value : u32) -> CodePoint { CodePoint :: from_u32 (value) . unwrap () } assert_eq ! (c (0) . to_u32 () , 0) ; assert_eq ! (c (0xD800) . to_u32 () , 0xD800) ; assert_eq ! (c (0x10FFFF) . to_u32 () , 0x10FFFF) ; }
};
}
