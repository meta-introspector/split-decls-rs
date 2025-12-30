// Generated macro for wtf8_from_str (function)
macro_rules! Depcrate_wtf8_testswtf8_from_str {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_from_str"}
// Dependencies: {}
# [test] fn wtf8_from_str () { assert_eq ! (& Wtf8 :: from_str ("") . as_bytes () , b"") ; assert_eq ! (& Wtf8 :: from_str ("aé 💩") . as_bytes () , b"a\xC3\xA9 \xF0\x9F\x92\xA9") ; }
};
}
