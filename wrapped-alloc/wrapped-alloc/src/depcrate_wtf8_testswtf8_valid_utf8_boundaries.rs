// Generated macro for wtf8_valid_utf8_boundaries (function)
macro_rules! Depcrate_wtf8_testswtf8_valid_utf8_boundaries {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_valid_utf8_boundaries"}
// Dependencies: {}
# [test] fn wtf8_valid_utf8_boundaries () { let mut string = Wtf8Buf :: from_str ("aé 💩") ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . check_utf8_boundary (0) ; string . check_utf8_boundary (1) ; string . check_utf8_boundary (3) ; string . check_utf8_boundary (4) ; string . check_utf8_boundary (8) ; string . check_utf8_boundary (14) ; assert_eq ! (string . len () , 14) ; string . push_char ('a') ; string . check_utf8_boundary (14) ; string . check_utf8_boundary (15) ; let mut string = Wtf8Buf :: from_str ("a") ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . check_utf8_boundary (1) ; let mut string = Wtf8Buf :: from_str ("\u{D7FF}") ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . check_utf8_boundary (3) ; let mut string = Wtf8Buf :: new () ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . push_char ('\u{D7FF}') ; string . check_utf8_boundary (3) ; }
};
}
