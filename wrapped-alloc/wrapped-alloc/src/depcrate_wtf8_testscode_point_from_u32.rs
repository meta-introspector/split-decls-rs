// Generated macro for code_point_from_u32 (function)
macro_rules! Depcrate_wtf8_testscode_point_from_u32 {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_from_u32"}
// Dependencies: {}
# [test] fn code_point_from_u32 () { assert ! (CodePoint :: from_u32 (0) . is_some ()) ; assert ! (CodePoint :: from_u32 (0xD800) . is_some ()) ; assert ! (CodePoint :: from_u32 (0x10FFFF) . is_some ()) ; assert ! (CodePoint :: from_u32 (0x110000) . is_none ()) ; }
};
}
