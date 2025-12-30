// Generated macro for wtf8buf_into_string (function)
macro_rules! Depcrate_wtf8_testswtf8buf_into_string {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_into_string"}
// Dependencies: {}
# [test] fn wtf8buf_into_string () { let mut string = Wtf8Buf :: from_str ("aé 💩") ; assert ! (string . is_known_utf8) ; assert_eq ! (string . clone () . into_string () , Ok (String :: from ("aé 💩"))) ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; assert ! (! string . is_known_utf8) ; assert_eq ! (string . clone () . into_string () , Err (string)) ; }
};
}
