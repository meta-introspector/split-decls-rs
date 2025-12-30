// Generated macro for test_utf8 (function)
macro_rules! Depcrate_tests_stringtest_utf8 {
() => {
// Module: crate::tests::string
// Provides: {"test_utf8"}
// Dependencies: {}
# [test] fn test_utf8 () { let expected = "ประเทศไทย中华Việt Nam" ; let s = NSString :: from_str (expected) ; assert_eq ! (s . len () , expected . len ()) ; autoreleasepool (| pool | unsafe { assert_eq ! (s . to_str (pool) , expected) ; }) ; assert_eq ! (s . to_string () , expected) ; }
};
}
