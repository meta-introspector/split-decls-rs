// Generated macro for test_set (function)
macro_rules! Depcrate_tests_mutable_stringtest_set {
() => {
// Module: crate::tests::mutable_string
// Provides: {"test_set"}
// Dependencies: {}
# [test] fn test_set () { let s = NSMutableString :: from_str ("abc") ; s . setString (& NSString :: from_str ("def")) ; assert_eq ! (& s . to_string () , "def") ; }
};
}
