// Generated macro for build_dir_ignored_path_patterns (function)
macro_rules! Depcrate_pathsbuild_dir_ignored_path_patterns {
() => {
// Module: crate::paths
// Provides: {"build_dir_ignored_path_patterns"}
// Dependencies: {}
# [doc = " The paths to ignore when [`CargoPathExt::assert_build_dir_layout`] is called"] fn build_dir_ignored_path_patterns () -> Vec < String > { vec ! ["[..].dSYM/[..]" , "[..].pdb" ,] . into_iter () . map (ToString :: to_string) . collect () }
};
}
