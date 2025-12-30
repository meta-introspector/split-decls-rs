// Generated macro for covmap_var_name (function)
macro_rules! Depcrate_coverageinfo_llvm_covcovmap_var_name {
() => {
// Module: crate::coverageinfo::llvm_cov
// Provides: {"covmap_var_name"}
// Dependencies: {}
pub (crate) fn covmap_var_name () -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovmapVarNameToString (s) ; })) . expect ("covmap variable name should not contain NUL") }
};
}
