// Generated macro for all_tests_discarded_min_tests_passed_missing (function)
macro_rules! Depcrate_testsall_tests_discarded_min_tests_passed_missing {
() => {
// Module: crate::tests
// Provides: {"all_tests_discarded_min_tests_passed_missing"}
// Dependencies: {}
# [test] fn all_tests_discarded_min_tests_passed_missing () { fn prop_discarded (_ : u8) -> TestResult { TestResult :: discard () } QuickCheck :: new () . quickcheck (prop_discarded as fn (u8) -> TestResult) ; }
};
}
