// Generated macro for wtf8_to_owned (function)
macro_rules! Depcrate_wtf8_testswtf8_to_owned {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_to_owned"}
// Dependencies: {}
# [test] fn wtf8_to_owned () { let string = to_owned (unsafe { Wtf8 :: from_bytes_unchecked (b"\xED\xA0\x80") }) ; assert_eq ! (string . as_bytes () , b"\xED\xA0\x80") ; assert ! (! string . is_known_utf8) ; }
};
}
