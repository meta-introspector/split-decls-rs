// Generated macro for qc_min_tests_passed (function)
macro_rules! Depcrate_testerqc_min_tests_passed {
() => {
// Module: crate::tester
// Provides: {"qc_min_tests_passed"}
// Dependencies: {}
fn qc_min_tests_passed () -> u64 { let default = 0 ; match env :: var ("QUICKCHECK_MIN_TESTS_PASSED") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
};
}
