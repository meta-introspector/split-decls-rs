// Generated macro for wtf8buf_show (function)
macro_rules! Depcrate_wtf8_testswtf8buf_show {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_show"}
// Dependencies: {}
# [test] fn wtf8buf_show () { let mut string = Wtf8Buf :: from_str ("a\té \u{7f}💩\r") ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; assert_eq ! (format ! ("{string:?}") , "\"a\\té \\u{7f}\u{1f4a9}\\r\\u{d800}\"") ; }
};
}
