// Generated macro for test_from_nsstring (function)
macro_rules! Depcrate_tests_mutable_stringtest_from_nsstring {
() => {
// Module: crate::tests::mutable_string
// Provides: {"test_from_nsstring"}
// Dependencies: {}
# [test] fn test_from_nsstring () { let s = NSString :: from_str ("abc") ; let s = NSMutableString :: stringWithString (& s) ; assert_eq ! (& s . to_string () , "abc") ; }
};
}
