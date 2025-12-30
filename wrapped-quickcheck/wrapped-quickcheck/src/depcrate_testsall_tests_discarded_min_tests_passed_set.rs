// Generated macro for all_tests_discarded_min_tests_passed_set (function)
macro_rules! Depcrate_testsall_tests_discarded_min_tests_passed_set {
() => {
// Module: crate::tests
// Provides: {"all_tests_discarded_min_tests_passed_set"}
// Dependencies: {}
# [test] # [should_panic (expected = "(Unable to generate enough tests, 0 not discarded.)")] fn all_tests_discarded_min_tests_passed_set () { fn prop_discarded (_ : u8) -> TestResult { TestResult :: discard () } QuickCheck :: new () . tests (16) . min_tests_passed (8) . quickcheck (prop_discarded as fn (u8) -> TestResult) ; }
};
}
