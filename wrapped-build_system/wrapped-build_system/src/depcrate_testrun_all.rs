// Generated macro for run_all (function)
macro_rules! Depcrate_testrun_all {
() => {
// Module: crate::test
// Provides: {"run_all"}
// Dependencies: {}
fn run_all (env : & Env , args : & TestArg) -> Result < () , String > { clean (env , args) ? ; mini_tests (env , args) ? ; build_sysroot (env , args) ? ; std_tests (env , args) ? ; test_libcore (env , args) ? ; extended_sysroot_tests (env , args) ? ; cargo_tests (env , args) ? ; test_rustc (env , args) ? ; Ok (()) }
};
}
