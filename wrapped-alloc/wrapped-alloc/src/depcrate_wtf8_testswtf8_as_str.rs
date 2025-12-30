// Generated macro for wtf8_as_str (function)
macro_rules! Depcrate_wtf8_testswtf8_as_str {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_as_str"}
// Dependencies: {}
# [test] fn wtf8_as_str () { assert_eq ! (Wtf8 :: from_str ("") . as_str () , Ok ("")) ; assert_eq ! (Wtf8 :: from_str ("aé 💩") . as_str () , Ok ("aé 💩")) ; let mut string = Wtf8Buf :: new () ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; assert ! (string . as_str () . is_err ()) ; }
};
}
