// Generated macro for test_prefix_suffix (function)
macro_rules! Depcrate_tests_stringtest_prefix_suffix {
() => {
// Module: crate::tests::string
// Provides: {"test_prefix_suffix"}
// Dependencies: {}
# [test] fn test_prefix_suffix () { let s = NSString :: from_str ("abcdef") ; let prefix = NSString :: from_str ("abc") ; let suffix = NSString :: from_str ("def") ; assert ! (s . hasPrefix (& prefix)) ; assert ! (s . hasSuffix (& suffix)) ; assert ! (! s . hasPrefix (& suffix)) ; assert ! (! s . hasSuffix (& prefix)) ; }
};
}
