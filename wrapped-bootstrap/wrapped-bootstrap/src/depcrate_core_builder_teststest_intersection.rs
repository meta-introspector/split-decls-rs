// Generated macro for test_intersection (function)
macro_rules! Depcrate_core_builder_teststest_intersection {
() => {
// Module: crate::core::builder::tests
// Provides: {"test_intersection"}
// Dependencies: {}
# [test] fn test_intersection () { let set = | paths : & [& str] | { PathSet :: Set (paths . into_iter () . map (| p | TaskPath { path : p . into () , kind : None }) . collect ()) } ; let library_set = set (& ["library/core" , "library/alloc" , "library/std"]) ; let mut command_paths = vec ! [CLIStepPath :: from (PathBuf :: from ("library/core")) , CLIStepPath :: from (PathBuf :: from ("library/alloc")) , CLIStepPath :: from (PathBuf :: from ("library/stdarch")) ,] ; let subset = library_set . intersection_removing_matches (& mut command_paths , Kind :: Build) ; assert_eq ! (subset , set (& ["library/core" , "library/alloc"]) ,) ; assert_eq ! (command_paths , vec ! [CLIStepPath :: from (PathBuf :: from ("library/core")) . will_be_executed (true) , CLIStepPath :: from (PathBuf :: from ("library/alloc")) . will_be_executed (true) , CLIStepPath :: from (PathBuf :: from ("library/stdarch")) . will_be_executed (false) ,]) ; }
};
}
