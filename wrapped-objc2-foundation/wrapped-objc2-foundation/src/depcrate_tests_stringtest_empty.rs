// Generated macro for test_empty (function)
macro_rules! Depcrate_tests_stringtest_empty {
() => {
// Module: crate::tests::string
// Provides: {"test_empty"}
// Dependencies: {}
# [test] fn test_empty () { let s1 = NSString :: from_str ("") ; let s2 = NSString :: new () ; assert_eq ! (s1 . len () , 0) ; assert_eq ! (s2 . len () , 0) ; assert_eq ! (s1 , s2) ; autoreleasepool (| pool | unsafe { assert_eq ! (s1 . to_str (pool) , "") ; assert_eq ! (s2 . to_str (pool) , "") ; }) ; assert_eq ! (s1 . to_string () , "") ; assert_eq ! (s2 . to_string () , "") ; }
};
}
