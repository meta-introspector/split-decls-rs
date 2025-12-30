// Generated macro for wtf8_make_ascii_lowercase (function)
macro_rules! Depcrate_wtf8_testswtf8_make_ascii_lowercase {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_make_ascii_lowercase"}
// Dependencies: {}
# [test] fn wtf8_make_ascii_lowercase () { let mut lowercase = Wtf8Buf :: from_str ("") ; lowercase . make_ascii_lowercase () ; assert_eq ! (lowercase . as_bytes () , b"") ; let mut lowercase = Wtf8Buf :: from_str ("GrEeN gRaPeS! 🍇") ; lowercase . make_ascii_lowercase () ; assert_eq ! (lowercase . as_bytes () , b"green grapes! \xf0\x9f\x8d\x87") ; let mut lowercase = to_owned (unsafe { Wtf8 :: from_bytes_unchecked (b"\xED\xA0\x80") }) ; lowercase . make_ascii_lowercase () ; assert_eq ! (lowercase . as_bytes () , b"\xED\xA0\x80") ; assert ! (! lowercase . is_known_utf8) ; }
};
}
