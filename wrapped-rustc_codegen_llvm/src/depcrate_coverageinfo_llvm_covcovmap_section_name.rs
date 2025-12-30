// Generated macro for covmap_section_name (function)
macro_rules! Depcrate_coverageinfo_llvm_covcovmap_section_name {
() => {
// Module: crate::coverageinfo::llvm_cov
// Provides: {"covmap_section_name"}
// Dependencies: {}
pub (crate) fn covmap_section_name (llmod : & llvm :: Module) -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovmapSectionNameToString (llmod , s) ; })) . expect ("covmap section name should not contain NUL") }
};
}
