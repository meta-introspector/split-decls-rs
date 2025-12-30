// Generated macro for wtf8buf_push_char (function)
macro_rules! Depcrate_wtf8_testswtf8buf_push_char {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_push_char"}
// Dependencies: {}
# [test] fn wtf8buf_push_char () { let mut string = Wtf8Buf :: from_str ("aé ") ; assert_eq ! (string . as_bytes () , b"a\xC3\xA9 ") ; assert ! (string . is_known_utf8) ; string . push_char ('💩') ; assert_eq ! (string . as_bytes () , b"a\xC3\xA9 \xF0\x9F\x92\xA9") ; assert ! (string . is_known_utf8) ; }
};
}
