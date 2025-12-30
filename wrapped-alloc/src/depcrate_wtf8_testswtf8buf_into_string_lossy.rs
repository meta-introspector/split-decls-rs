// Generated macro for wtf8buf_into_string_lossy (function)
macro_rules! Depcrate_wtf8_testswtf8buf_into_string_lossy {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_into_string_lossy"}
// Dependencies: {}
# [test] fn wtf8buf_into_string_lossy () { let mut string = Wtf8Buf :: from_str ("aé 💩") ; assert_eq ! (string . clone () . into_string_lossy () , String :: from ("aé 💩")) ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; assert_eq ! (string . clone () . into_string_lossy () , String :: from ("aé 💩�")) ; }
};
}
