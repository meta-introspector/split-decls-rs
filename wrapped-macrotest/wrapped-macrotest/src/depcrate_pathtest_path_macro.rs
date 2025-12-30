// Generated macro for test_path_macro (function)
macro_rules! Depcrate_pathtest_path_macro {
() => {
// Module: crate::path
// Provides: {"test_path_macro"}
// Dependencies: {}
# [test] fn test_path_macro () { use std :: path :: { Path , PathBuf } ; struct Project { dir : PathBuf , } let project = Project { dir : PathBuf :: from ("../target/tests") , } ; let cargo_dir = path ! (project . dir / ".cargo" / "config.toml") ; assert_eq ! (cargo_dir , Path :: new ("../target/tests/.cargo/config.toml")) ; }
};
}
