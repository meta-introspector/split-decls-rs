// Generated macro for wtf8buf_truncate_around_non_bmp (function)
macro_rules! Depcrate_wtf8_testswtf8buf_truncate_around_non_bmp {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_truncate_around_non_bmp"}
// Dependencies: {}
# [test] fn wtf8buf_truncate_around_non_bmp () { let mut string = Wtf8Buf :: from_str ("💩") ; assert ! (string . is_known_utf8) ; string . truncate (4) ; assert_eq ! (string . as_bytes () , b"\xF0\x9F\x92\xA9") ; assert ! (string . is_known_utf8) ; string . truncate (0) ; assert_eq ! (string . as_bytes () , b"") ; assert ! (string . is_known_utf8) ; }
};
}
