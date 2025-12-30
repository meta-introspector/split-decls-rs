// Generated macro for wtf8_make_ascii_uppercase (function)
macro_rules! Depcrate_wtf8_testswtf8_make_ascii_uppercase {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_make_ascii_uppercase"}
// Dependencies: {}
# [test] fn wtf8_make_ascii_uppercase () { let mut uppercase = Wtf8Buf :: from_str ("") ; uppercase . make_ascii_uppercase () ; assert_eq ! (uppercase . as_bytes () , b"") ; let mut uppercase = Wtf8Buf :: from_str ("GrEeN gRaPeS! 🍇") ; uppercase . make_ascii_uppercase () ; assert_eq ! (uppercase . as_bytes () , b"GREEN GRAPES! \xf0\x9f\x8d\x87") ; let mut uppercase = to_owned (unsafe { Wtf8 :: from_bytes_unchecked (b"\xED\xA0\x80") }) ; uppercase . make_ascii_uppercase () ; assert_eq ! (uppercase . as_bytes () , b"\xED\xA0\x80") ; assert ! (! uppercase . is_known_utf8) ; }
};
}
