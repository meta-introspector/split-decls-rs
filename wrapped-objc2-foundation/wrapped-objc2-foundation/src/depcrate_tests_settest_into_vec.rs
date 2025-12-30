// Generated macro for test_into_vec (function)
macro_rules! Depcrate_tests_settest_into_vec {
() => {
// Module: crate::tests::set
// Provides: {"test_into_vec"}
// Dependencies: {}
# [test] fn test_into_vec () { let strs = [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")] ; let set = NSSet :: from_slice (& strs) ; assert_eq ! (set . len () , 3) ; assert_eq ! (set . to_vec () . len () , 3) ; }
};
}
