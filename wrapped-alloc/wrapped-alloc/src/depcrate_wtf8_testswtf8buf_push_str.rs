// Generated macro for wtf8buf_push_str (function)
macro_rules! Depcrate_wtf8_testswtf8buf_push_str {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_push_str"}
// Dependencies: {}
# [test] fn wtf8buf_push_str () { let mut string = Wtf8Buf :: new () ; assert_eq ! (string . as_bytes () , b"") ; assert ! (string . is_known_utf8) ; string . push_str ("aé 💩") ; assert_eq ! (string . as_bytes () , b"a\xC3\xA9 \xF0\x9F\x92\xA9") ; assert ! (string . is_known_utf8) ; }
};
}
