// Generated macro for CargoBuild (struct)
macro_rules! Depcrate_buildCargoBuild {
() => {
// Module: crate::build
// Provides: {"CargoBuild"}
// Dependencies: {}
# [doc = " The `build` subcommand."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # let target_dir = tempfile::TempDir::new().unwrap();"] # [doc = " escargot::CargoBuild::new()"] # [doc = "     .bin(\"bin\")"] # [doc = "     .current_release()"] # [doc = "     .current_target()"] # [doc = "     .manifest_path(\"tests/testsuite/fixtures/bin/Cargo.toml\")"] # [doc = "     .target_dir(target_dir.path())"] # [doc = "     .exec()"] # [doc = "     .unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct CargoBuild { cmd : process :: Command , bin : bool , example : bool , }
};
}
