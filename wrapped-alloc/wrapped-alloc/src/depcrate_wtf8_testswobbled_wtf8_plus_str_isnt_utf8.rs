// Generated macro for wobbled_wtf8_plus_str_isnt_utf8 (function)
macro_rules! Depcrate_wtf8_testswobbled_wtf8_plus_str_isnt_utf8 {
() => {
// Module: crate::wtf8::tests
// Provides: {"wobbled_wtf8_plus_str_isnt_utf8"}
// Dependencies: {}
# [test] fn wobbled_wtf8_plus_str_isnt_utf8 () { let mut string : Wtf8Buf = to_owned (unsafe { Wtf8 :: from_bytes_unchecked (b"\xED\xA0\x80") }) ; assert ! (! string . is_known_utf8) ; string . push_str ("some utf-8") ; assert ! (! string . is_known_utf8) ; }
};
}
