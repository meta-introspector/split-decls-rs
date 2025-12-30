// Generated macro for test_from_nsstring (function)
macro_rules! Depcrate_tests_attributed_stringtest_from_nsstring {
() => {
// Module: crate::tests::attributed_string
// Provides: {"test_from_nsstring"}
// Dependencies: {}
# [test] fn test_from_nsstring () { let s = NSAttributedString :: from_nsstring (ns_string ! ("abc")) ; assert_eq ! (& s . string () . to_string () , "abc") ; }
};
}
