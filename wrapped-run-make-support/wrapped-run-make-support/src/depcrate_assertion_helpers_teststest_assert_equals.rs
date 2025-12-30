// Generated macro for test_assert_equals (module)
macro_rules! Depcrate_assertion_helpers_teststest_assert_equals {
() => {
// Module: crate::assertion_helpers::tests
// Provides: {"test_assert_equals"}
// Dependencies: {}
mod test_assert_equals { use super :: * ; # [test] fn assert_equals_same () { assert_equals ("foo" , "foo") ; assert_equals ("" , "") ; } # [test] # [should_panic] fn assert_equals_different () { assert_equals ("foo" , "bar") ; } }
};
}
