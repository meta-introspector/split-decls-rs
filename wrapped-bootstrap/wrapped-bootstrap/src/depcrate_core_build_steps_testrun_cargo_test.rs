// Generated macro for run_cargo_test (function)
macro_rules! Depcrate_core_build_steps_testrun_cargo_test {
() => {
// Module: crate::core::build_steps::test
// Provides: {"run_cargo_test"}
// Dependencies: {}
# [doc = " Given a `cargo test` subcommand, add the appropriate flags and run it."] # [doc = ""] # [doc = " Returns whether the test succeeded."] fn run_cargo_test < 'a > (cargo : builder :: Cargo , libtest_args : & [& str] , crates : & [String] , description : impl Into < Option < & 'a str > > , target : TargetSelection , builder : & Builder < '_ > ,) -> bool { let compiler = cargo . compiler () ; let mut cargo = prepare_cargo_test (cargo , libtest_args , crates , target , builder) ; let _time = helpers :: timeit (builder) ; let _group = description . into () . and_then (| what | builder . msg_test (what , target , compiler . stage + 1)) ; # [cfg (feature = "build-metrics")] builder . metrics . begin_test_suite (build_helper :: metrics :: TestSuiteMetadata :: CargoPackage { crates : crates . iter () . map (| c | c . to_string ()) . collect () , target : target . triple . to_string () , host : compiler . host . triple . to_string () , stage : compiler . stage , } , builder ,) ; add_flags_and_try_run_tests (builder , & mut cargo) }
};
}
