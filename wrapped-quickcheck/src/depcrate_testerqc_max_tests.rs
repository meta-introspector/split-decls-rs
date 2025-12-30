// Generated macro for qc_max_tests (function)
macro_rules! Depcrate_testerqc_max_tests {
() => {
// Module: crate::tester
// Provides: {"qc_max_tests"}
// Dependencies: {}
fn qc_max_tests () -> u64 { let default = 10_000 ; match env :: var ("QUICKCHECK_MAX_TESTS") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
};
}
