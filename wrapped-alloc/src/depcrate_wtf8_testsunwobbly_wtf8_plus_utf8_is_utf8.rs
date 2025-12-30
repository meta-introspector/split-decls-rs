// Generated macro for unwobbly_wtf8_plus_utf8_is_utf8 (function)
macro_rules! Depcrate_wtf8_testsunwobbly_wtf8_plus_utf8_is_utf8 {
() => {
// Module: crate::wtf8::tests
// Provides: {"unwobbly_wtf8_plus_utf8_is_utf8"}
// Dependencies: {}
# [test] fn unwobbly_wtf8_plus_utf8_is_utf8 () { let mut string : Wtf8Buf = Wtf8Buf :: from_str ("hello world") ; assert ! (string . is_known_utf8) ; string . push_str ("some utf-8") ; assert ! (string . is_known_utf8) ; }
};
}
