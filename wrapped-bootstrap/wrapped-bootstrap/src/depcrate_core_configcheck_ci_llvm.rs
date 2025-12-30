// Generated macro for check_ci_llvm (macro)
macro_rules! Depcrate_core_configcheck_ci_llvm {
() => {
// Module: crate::core::config
// Provides: {"check_ci_llvm"}
// Dependencies: {}
# [macro_export] macro_rules ! check_ci_llvm { ($ name : expr) => { assert ! ($ name . is_none () , "setting {} is incompatible with download-ci-llvm." , stringify ! ($ name) . replace ("_" , "-")) ; } ; }
};
}
