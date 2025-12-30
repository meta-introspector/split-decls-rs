// Generated macro for qc_tests (function)
macro_rules! Depcrate_testerqc_tests {
() => {
// Module: crate::tester
// Provides: {"qc_tests"}
// Dependencies: {}
fn qc_tests () -> u64 { let default = 100 ; match env :: var ("QUICKCHECK_TESTS") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
};
}
