// Generated macro for test_two_iters (function)
macro_rules! Depcrate_tests_arraytest_two_iters {
() => {
// Module: crate::tests::array
// Provides: {"test_two_iters"}
// Dependencies: {}
# [test] fn test_two_iters () { let array = sample_number_array (4) ; let iter1 = array . iter () ; let iter2 = array . iter () ; for (_ , _) in iter1 . zip (iter2) { } }
};
}
