// Generated macro for qc_gen_size (function)
macro_rules! Depcrate_testerqc_gen_size {
() => {
// Module: crate::tester
// Provides: {"qc_gen_size"}
// Dependencies: {}
fn qc_gen_size () -> usize { let default = 100 ; match env :: var ("QUICKCHECK_GENERATOR_SIZE") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
};
}
