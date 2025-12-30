// Generated macro for detect_gcc_freshness (function)
macro_rules! Depcrate_core_build_steps_gccdetect_gcc_freshness {
() => {
// Module: crate::core::build_steps::gcc
// Provides: {"detect_gcc_freshness"}
// Dependencies: {}
# [doc = " Detect whether GCC sources have been modified locally or not."] # [cfg (not (test))] fn detect_gcc_freshness (config : & crate :: Config , is_git : bool) -> build_helper :: git :: PathFreshness { use build_helper :: git :: PathFreshness ; if is_git { config . check_path_modifications (& ["src/gcc" , "src/bootstrap/download-ci-gcc-stamp"]) } else if let Some (info) = crate :: utils :: channel :: read_commit_info_file (& config . src) { PathFreshness :: LastModifiedUpstream { upstream : info . sha . trim () . to_owned () } } else { PathFreshness :: MissingUpstream } }
};
}
