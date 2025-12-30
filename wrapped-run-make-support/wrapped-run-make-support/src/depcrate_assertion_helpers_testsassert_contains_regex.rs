// Generated macro for assert_contains_regex (module)
macro_rules! Depcrate_assertion_helpers_testsassert_contains_regex {
() => {
// Module: crate::assertion_helpers::tests
// Provides: {"assert_contains_regex"}
// Dependencies: {}
mod assert_contains_regex { use super :: * ; # [test] fn assert_contains_regex_yes () { assert_contains_regex ("" , "") ; assert_contains_regex ("" , ".*") ; assert_contains_regex ("abcde" , ".*") ; assert_contains_regex ("abcde" , ".+") ; } # [test] # [should_panic] fn assert_contains_regex_no () { assert_contains_regex ("" , ".+") ; } }
};
}
