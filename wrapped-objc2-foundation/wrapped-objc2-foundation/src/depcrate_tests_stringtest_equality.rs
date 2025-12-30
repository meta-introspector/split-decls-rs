// Generated macro for test_equality (function)
macro_rules! Depcrate_tests_stringtest_equality {
() => {
// Module: crate::tests::string
// Provides: {"test_equality"}
// Dependencies: {}
# [test] fn test_equality () { let s1 = NSString :: from_str ("abc") ; let s2 = NSString :: from_str ("abc") ; assert_eq ! (s1 , s1) ; assert_eq ! (s1 , s2) ; let s3 = NSString :: from_str ("def") ; assert_ne ! (s1 , s3) ; }
};
}
