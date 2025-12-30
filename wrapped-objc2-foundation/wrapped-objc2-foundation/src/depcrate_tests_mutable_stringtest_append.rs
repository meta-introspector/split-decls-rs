// Generated macro for test_append (function)
macro_rules! Depcrate_tests_mutable_stringtest_append {
() => {
// Module: crate::tests::mutable_string
// Provides: {"test_append"}
// Dependencies: {}
# [test] # [allow (clippy :: deref_addrof)] fn test_append () { let s = NSMutableString :: from_str ("abc") ; s . appendString (& NSString :: from_str ("def")) ; * & mut & * s += & NSString :: from_str ("ghi") ; assert_eq ! (& s . to_string () , "abcdefghi") ; }
};
}
