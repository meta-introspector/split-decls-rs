// Generated macro for wtf8buf_truncate (function)
macro_rules! Depcrate_wtf8_testswtf8buf_truncate {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_truncate"}
// Dependencies: {}
# [test] fn wtf8buf_truncate () { let mut string = Wtf8Buf :: from_str ("aé") ; assert ! (string . is_known_utf8) ; string . truncate (3) ; assert_eq ! (string . as_bytes () , b"a\xC3\xA9") ; assert ! (string . is_known_utf8) ; string . truncate (1) ; assert_eq ! (string . as_bytes () , b"a") ; assert ! (string . is_known_utf8) ; string . truncate (0) ; assert_eq ! (string . as_bytes () , b"") ; assert ! (string . is_known_utf8) ; }
};
}
