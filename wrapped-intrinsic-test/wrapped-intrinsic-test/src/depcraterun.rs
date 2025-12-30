// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (test_environment : impl SupportedArchitectureTest) { info ! ("building C binaries") ; if ! test_environment . build_c_file () { std :: process :: exit (2) ; } info ! ("building Rust binaries") ; if ! test_environment . build_rust_file () { std :: process :: exit (3) ; } info ! ("Running binaries") ; if ! test_environment . compare_outputs () { std :: process :: exit (1) ; } }
};
}
