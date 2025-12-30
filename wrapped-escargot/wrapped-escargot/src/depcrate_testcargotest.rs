// Generated macro for CargoTest (struct)
macro_rules! Depcrate_testCargoTest {
() => {
// Module: crate::test
// Provides: {"CargoTest"}
// Dependencies: {}
# [doc = " The `test` subcommand (emulated)."] # [doc = ""] # [doc = " Created via [`CargoBuild::run_tests`]."] # [doc = ""] # [doc = " Benefits over spawning `cargo test`:"] # [doc = " - Able to cache binary path, avoiding cargo overhead."] # [doc = " - Independent of CWD."] # [doc = " - stdout/stderr are clean of `cargo test` output."] # [doc = ""] # [doc = " Required feature: `test_unstable` since the format parsed is unstable."] # [doc = ""] # [doc = " Relevant features"] # [doc = " - `print` for logged output to be printed instead, generally for test writing."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # let target_dir = tempfile::TempDir::new().unwrap();"] # [doc = " let run = escargot::CargoBuild::new()"] # [doc = "     .test(\"test\")"] # [doc = "     .manifest_path(\"tests/testsuite/fixtures/test/Cargo.toml\")"] # [doc = "     .target_dir(target_dir.path())"] # [doc = "     .run_tests().unwrap()"] # [doc = "     .next().unwrap().unwrap();"] # [doc = " println!(\"artifact={}\", run.path().display());"] # [doc = " ```"] # [doc = ""] # [doc = " [`CargoBuild::run_tests`]: crate::CargoBuild::run_tests()"] # [derive (Debug)] pub struct CargoTest { bin_path : path :: PathBuf , kind : String , name : String , }
};
}
