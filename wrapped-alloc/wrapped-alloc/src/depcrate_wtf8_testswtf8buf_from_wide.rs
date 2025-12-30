// Generated macro for wtf8buf_from_wide (function)
macro_rules! Depcrate_wtf8_testswtf8buf_from_wide {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_from_wide"}
// Dependencies: {}
# [test] fn wtf8buf_from_wide () { let buf = Wtf8Buf :: from_wide (& []) ; assert_eq ! (buf . as_bytes () , b"") ; assert ! (buf . is_known_utf8) ; let buf = Wtf8Buf :: from_wide (& [0x61 , 0xE9 , 0x20 , 0xD83D , 0xDCA9]) ; assert_eq ! (buf . as_bytes () , b"a\xC3\xA9 \xF0\x9F\x92\xA9") ; assert ! (buf . is_known_utf8) ; let buf = Wtf8Buf :: from_wide (& [0x61 , 0xE9 , 0x20 , 0xD83D , 0xD83D , 0xDCA9]) ; assert_eq ! (buf . as_bytes () , b"a\xC3\xA9 \xED\xA0\xBD\xF0\x9F\x92\xA9") ; assert ! (! buf . is_known_utf8) ; let buf = Wtf8Buf :: from_wide (& [0xD800]) ; assert_eq ! (buf . as_bytes () , b"\xED\xA0\x80") ; assert ! (! buf . is_known_utf8) ; let buf = Wtf8Buf :: from_wide (& [0xDBFF]) ; assert_eq ! (buf . as_bytes () , b"\xED\xAF\xBF") ; assert ! (! buf . is_known_utf8) ; let buf = Wtf8Buf :: from_wide (& [0xDC00]) ; assert_eq ! (buf . as_bytes () , b"\xED\xB0\x80") ; assert ! (! buf . is_known_utf8) ; let buf = Wtf8Buf :: from_wide (& [0xDFFF]) ; assert_eq ! (buf . as_bytes () , b"\xED\xBF\xBF") ; assert ! (! buf . is_known_utf8) ; }
};
}
