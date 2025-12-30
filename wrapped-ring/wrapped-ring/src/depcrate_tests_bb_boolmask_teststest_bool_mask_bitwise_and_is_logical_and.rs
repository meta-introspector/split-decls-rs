// Generated macro for test_bool_mask_bitwise_and_is_logical_and (function)
macro_rules! Depcrate_tests_bb_boolmask_teststest_bool_mask_bitwise_and_is_logical_and {
() => {
// Module: crate::tests::bb_boolmask_tests
// Provides: {"test_bool_mask_bitwise_and_is_logical_and"}
// Dependencies: {}
# [test] fn test_bool_mask_bitwise_and_is_logical_and () { assert ! (leak_in_test (BoolMask :: TRUE & BoolMask :: TRUE)) ; assert ! (! leak_in_test (BoolMask :: TRUE & BoolMask :: FALSE)) ; assert ! (! leak_in_test (BoolMask :: FALSE & BoolMask :: TRUE)) ; assert ! (! leak_in_test (BoolMask :: FALSE & BoolMask :: FALSE)) ; }
};
}
