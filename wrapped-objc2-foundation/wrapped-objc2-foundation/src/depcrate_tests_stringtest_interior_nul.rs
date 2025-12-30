// Generated macro for test_interior_nul (function)
macro_rules! Depcrate_tests_stringtest_interior_nul {
() => {
// Module: crate::tests::string
// Provides: {"test_interior_nul"}
// Dependencies: {}
# [test] fn test_interior_nul () { let expected = "Hello\0World" ; let s = NSString :: from_str (expected) ; assert_eq ! (s . len () , expected . len ()) ; autoreleasepool (| pool | unsafe { assert_eq ! (s . to_str (pool) , expected) ; }) ; assert_eq ! (s . to_string () , expected) ; }
};
}
