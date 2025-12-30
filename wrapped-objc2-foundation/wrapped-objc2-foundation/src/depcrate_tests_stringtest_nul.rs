// Generated macro for test_nul (function)
macro_rules! Depcrate_tests_stringtest_nul {
() => {
// Module: crate::tests::string
// Provides: {"test_nul"}
// Dependencies: {}
# [test] fn test_nul () { let expected = "\0" ; let s = NSString :: from_str (expected) ; assert_eq ! (s . len () , expected . len ()) ; autoreleasepool (| pool | unsafe { assert_eq ! (s . to_str (pool) , expected) ; }) ; assert_eq ! (s . to_string () , expected) ; }
};
}
