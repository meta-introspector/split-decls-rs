// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/llvm_cov.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::coverageinfo::ffi;
use crate::llvm;

pub(crate) fn covmap_var_name() -> CString {
    CString::new(llvm::build_byte_buffer(|s| unsafe {
        llvm::LLVMRustCoverageWriteCovmapVarNameToString(s);
    }))
