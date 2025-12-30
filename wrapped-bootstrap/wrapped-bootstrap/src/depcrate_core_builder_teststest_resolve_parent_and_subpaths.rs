// Generated macro for test_resolve_parent_and_subpaths (function)
macro_rules! Depcrate_core_builder_teststest_resolve_parent_and_subpaths {
() => {
// Module: crate::core::builder::tests
// Provides: {"test_resolve_parent_and_subpaths"}
// Dependencies: {}
# [test] fn test_resolve_parent_and_subpaths () { let set = | paths : & [& str] | { PathSet :: Set (paths . into_iter () . map (| p | TaskPath { path : p . into () , kind : None }) . collect ()) } ; let mut command_paths = vec ! [CLIStepPath :: from (PathBuf :: from ("src/tools/miri")) , CLIStepPath :: from (PathBuf :: from ("src/tools/miri/cargo-miri")) ,] ; let library_set = set (& ["src/tools/miri" , "src/tools/miri/cargo-miri"]) ; library_set . intersection_removing_matches (& mut command_paths , Kind :: Build) ; assert_eq ! (command_paths , vec ! [CLIStepPath :: from (PathBuf :: from ("src/tools/miri")) . will_be_executed (true) , CLIStepPath :: from (PathBuf :: from ("src/tools/miri/cargo-miri")) . will_be_executed (true) ,]) ; }
};
}
