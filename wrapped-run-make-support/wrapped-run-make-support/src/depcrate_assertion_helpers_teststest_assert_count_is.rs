// Generated macro for test_assert_count_is (module)
macro_rules! Depcrate_assertion_helpers_teststest_assert_count_is {
() => {
// Module: crate::assertion_helpers::tests
// Provides: {"test_assert_count_is"}
// Dependencies: {}
mod test_assert_count_is { use super :: * ; # [test] fn assert_count_is_yes () { assert_count_is (0 , "" , "b") ; assert_count_is (3 , "abcbdb" , "b") ; } # [test] # [should_panic] fn assert_count_is_no () { assert_count_is (2 , "abcbdb" , "b") ; } }
};
}
