// Generated macro for test_iter_fused (function)
macro_rules! Depcrate_tests_arraytest_iter_fused {
() => {
// Module: crate::tests::array
// Provides: {"test_iter_fused"}
// Dependencies: {}
# [test] fn test_iter_fused () { let array = sample_number_array (2) ; let mut iter = array . iter () ; assert_eq ! (iter . next () , Some (NSNumber :: new_u8 (0))) ; assert_eq ! (iter . next () , Some (NSNumber :: new_u8 (1))) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . next () , None) ; }
};
}
