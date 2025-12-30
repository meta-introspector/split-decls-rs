// Generated macro for wtf8buf_from_string (function)
macro_rules! Depcrate_wtf8_testswtf8buf_from_string {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_from_string"}
// Dependencies: {}
# [test] fn wtf8buf_from_string () { assert_eq ! (Wtf8Buf :: from_string (String :: from ("")) . as_bytes () , b"") ; assert_eq ! (Wtf8Buf :: from_string (String :: from ("aé 💩")) . as_bytes () , b"a\xC3\xA9 \xF0\x9F\x92\xA9") ; }
};
}
