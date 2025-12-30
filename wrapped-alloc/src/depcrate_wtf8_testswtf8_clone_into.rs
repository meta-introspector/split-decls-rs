// Generated macro for wtf8_clone_into (function)
macro_rules! Depcrate_wtf8_testswtf8_clone_into {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_clone_into"}
// Dependencies: {}
# [test] fn wtf8_clone_into () { let mut string = Wtf8Buf :: new () ; clone_into (Wtf8 :: from_str ("green") , & mut string) ; assert_eq ! (string . as_bytes () , b"green") ; let mut string = Wtf8Buf :: from_str ("green") ; clone_into (Wtf8 :: from_str ("") , & mut string) ; assert_eq ! (string . as_bytes () , b"") ; let mut string = Wtf8Buf :: from_str ("red") ; clone_into (Wtf8 :: from_str ("green") , & mut string) ; assert_eq ! (string . as_bytes () , b"green") ; let mut string = Wtf8Buf :: from_str ("green") ; clone_into (Wtf8 :: from_str ("red") , & mut string) ; assert_eq ! (string . as_bytes () , b"red") ; let mut string = Wtf8Buf :: from_str ("green") ; assert ! (string . is_known_utf8) ; clone_into (unsafe { Wtf8 :: from_bytes_unchecked (b"\xED\xA0\x80") } , & mut string) ; assert_eq ! (string . as_bytes () , b"\xED\xA0\x80") ; assert ! (! string . is_known_utf8) ; }
};
}
