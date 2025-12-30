// Generated macro for code_point_to_char_lossy (function)
macro_rules! Depcrate_wtf8_testscode_point_to_char_lossy {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_to_char_lossy"}
// Dependencies: {}
# [test] fn code_point_to_char_lossy () { fn c (value : u32) -> CodePoint { CodePoint :: from_u32 (value) . unwrap () } assert_eq ! (c (0x61) . to_char_lossy () , 'a') ; assert_eq ! (c (0x1F4A9) . to_char_lossy () , '💩') ; assert_eq ! (c (0xD800) . to_char_lossy () , '\u{FFFD}') ; }
};
}
