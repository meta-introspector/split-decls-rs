// Generated macro for covfun_section_name (function)
macro_rules! Depcrate_coverageinfo_llvm_covcovfun_section_name {
() => {
// Module: crate::coverageinfo::llvm_cov
// Provides: {"covfun_section_name"}
// Dependencies: {}
pub (crate) fn covfun_section_name (llmod : & llvm :: Module) -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovfunSectionNameToString (llmod , s) ; })) . expect ("covfun section name should not contain NUL") }
};
}
