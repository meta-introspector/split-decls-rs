// Generated macro for test_get_return_lifetime (function)
macro_rules! Depcrate_tests_settest_get_return_lifetime {
() => {
// Module: crate::tests::set
// Provides: {"test_get_return_lifetime"}
// Dependencies: {}
# [test] fn test_get_return_lifetime () { let set = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("two")]) ; let res = { let value = NSString :: from_str ("one") ; set . member (& value) } ; assert_eq ! (res , Some (ns_string ! ("one") . copy ())) ; }
};
}
