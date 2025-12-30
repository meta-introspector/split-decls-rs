// Generated macro for test_cmp (function)
macro_rules! Depcrate_tests_stringtest_cmp {
() => {
// Module: crate::tests::string
// Provides: {"test_cmp"}
// Dependencies: {}
# [test] # [allow (clippy :: nonminimal_bool)] # [cfg (feature = "NSObjCRuntime")] fn test_cmp () { let s1 = NSString :: from_str ("aa") ; assert ! (s1 <= s1) ; assert ! (s1 >= s1) ; let s2 = NSString :: from_str ("ab") ; assert ! (s1 < s2) ; assert ! (! (s1 > s2)) ; assert ! (s1 <= s2) ; assert ! (! (s1 >= s2)) ; let s3 = NSString :: from_str ("ba") ; assert ! (s1 < s3) ; assert ! (! (s1 > s3)) ; assert ! (s1 <= s3) ; assert ! (! (s1 >= s3)) ; assert ! (s2 < s3) ; assert ! (! (s2 > s3)) ; assert ! (s2 <= s3) ; assert ! (! (s2 >= s3)) ; let s = NSString :: from_str ("abc") ; let shorter = NSString :: from_str ("a") ; let longer = NSString :: from_str ("abcdef") ; assert ! (s > shorter) ; assert ! (s < longer) ; }
};
}
