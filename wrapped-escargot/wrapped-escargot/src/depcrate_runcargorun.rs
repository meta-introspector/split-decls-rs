// Generated macro for CargoRun (struct)
macro_rules! Depcrate_runCargoRun {
() => {
// Module: crate::run
// Provides: {"CargoRun"}
// Dependencies: {}
# [doc = " The `run` subcommand (emulated)."] # [doc = ""] # [doc = " Created via [`CargoBuild::run`][crate::CargoBuild::run]."] # [doc = ""] # [doc = " Benefits over spawning `cargo run`:"] # [doc = " - Able to cache binary path, avoiding cargo overhead."] # [doc = " - Independent of CWD."] # [doc = " - stdout/stderr are clean of `cargo run` output."] # [doc = ""] # [doc = " Relevant features"] # [doc = " - `print` for logged output to be printed instead, generally for test writing."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " To create a [`CargoRun`]:"] # [doc = " ```rust"] # [doc = " # let target_dir = tempfile::TempDir::new().unwrap();"] # [doc = " let run = escargot::CargoBuild::new()"] # [doc = "     .bin(\"bin\")"] # [doc = "     .current_release()"] # [doc = "     .current_target()"] # [doc = "     .manifest_path(\"tests/testsuite/fixtures/bin/Cargo.toml\")"] # [doc = "     .target_dir(target_dir.path())"] # [doc = "     .run()"] # [doc = "     .unwrap();"] # [doc = " println!(\"artifact={}\", run.path().display());"] # [doc = " ```"] # [doc = " See [`CargoRun::path`] for how to then run the newly compiled"] # [doc = " program."] # [derive (Debug)] pub struct CargoRun { bin_path : path :: PathBuf , }
};
}
