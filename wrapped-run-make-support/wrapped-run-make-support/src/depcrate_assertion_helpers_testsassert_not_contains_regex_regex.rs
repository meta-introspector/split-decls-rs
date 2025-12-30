// Generated macro for assert_not_contains_regex_regex (module)
macro_rules! Depcrate_assertion_helpers_testsassert_not_contains_regex_regex {
() => {
// Module: crate::assertion_helpers::tests
// Provides: {"assert_not_contains_regex_regex"}
// Dependencies: {}
mod assert_not_contains_regex_regex { use super :: * ; # [test] fn assert_not_contains_regex_yes () { assert_not_contains_regex ("abc" , "d") ; } # [test] # [should_panic] fn assert_not_contains_regex_no () { assert_not_contains_regex ("abc" , ".*") ; } }
};
}
