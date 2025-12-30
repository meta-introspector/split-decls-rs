// Generated macro for test_assert_contains (module)
macro_rules! Depcrate_assertion_helpers_teststest_assert_contains {
() => {
// Module: crate::assertion_helpers::tests
// Provides: {"test_assert_contains"}
// Dependencies: {}
mod test_assert_contains { use super :: * ; # [test] fn assert_contains_yes () { assert_contains ("" , "") ; assert_contains (" " , "") ; assert_contains ("a" , "a") ; assert_contains ("ab" , "a") ; } # [test] # [should_panic] fn assert_contains_no () { assert_contains ("a" , "b") ; } }
};
}
