// Generated macro for test_assert_not_contains (module)
macro_rules! Depcrate_assertion_helpers_teststest_assert_not_contains {
() => {
// Module: crate::assertion_helpers::tests
// Provides: {"test_assert_not_contains"}
// Dependencies: {}
mod test_assert_not_contains { use super :: * ; # [test] fn assert_not_contains_yes () { assert_not_contains ("a" , "b") ; } # [test] # [should_panic] fn assert_not_contains_no () { assert_not_contains (" " , "") ; } }
};
}
