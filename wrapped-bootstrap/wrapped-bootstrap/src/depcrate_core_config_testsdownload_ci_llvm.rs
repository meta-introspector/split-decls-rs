// Generated macro for download_ci_llvm (function)
macro_rules! Depcrate_core_config_testsdownload_ci_llvm {
() => {
// Module: crate::core::config::tests
// Provides: {"download_ci_llvm"}
// Dependencies: {}
# [test] fn download_ci_llvm () { let config = parse ("llvm.download-ci-llvm = false") ; assert ! (! config . llvm_from_ci) ; let if_unchanged_config = parse ("llvm.download-ci-llvm = \"if-unchanged\"") ; if if_unchanged_config . llvm_from_ci && if_unchanged_config . is_running_on_ci { let has_changes = if_unchanged_config . has_changes_from_upstream (LLVM_INVALIDATION_PATHS) ; assert ! (! has_changes , "CI LLVM can't be enabled with 'if-unchanged' while there are changes in LLVM submodule.") ; } }
};
}
