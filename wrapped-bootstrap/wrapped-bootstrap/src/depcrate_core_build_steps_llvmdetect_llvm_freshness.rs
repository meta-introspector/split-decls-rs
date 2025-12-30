// Generated macro for detect_llvm_freshness (function)
macro_rules! Depcrate_core_build_steps_llvmdetect_llvm_freshness {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"detect_llvm_freshness"}
// Dependencies: {}
# [doc = " Detect whether LLVM sources have been modified locally or not."] pub (crate) fn detect_llvm_freshness (config : & Config , is_git : bool) -> PathFreshness { if is_git { config . check_path_modifications (LLVM_INVALIDATION_PATHS) } else if let Some (info) = crate :: utils :: channel :: read_commit_info_file (& config . src) { PathFreshness :: LastModifiedUpstream { upstream : info . sha . trim () . to_owned () } } else { PathFreshness :: MissingUpstream } }
};
}
