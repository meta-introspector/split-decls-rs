// Generated macro for append_same_string (function)
macro_rules! Depcrate_tests_mutable_stringappend_same_string {
() => {
// Module: crate::tests::mutable_string
// Provides: {"append_same_string"}
// Dependencies: {}
# [doc = " Test that appending a NSMutableString to itself works (i.e. ensure that"] # [doc = " `appendString` is sound)."] # [test] fn append_same_string () { let string = NSMutableString :: from_str ("foo") ; let mut expected = String :: from ("foo") ; for i in 0 .. 3 { string . appendString (& string) ; expected += & * expected . clone () ; assert_eq ! (string . to_string () , expected , "failed at iteration {i}") ; } }
};
}
