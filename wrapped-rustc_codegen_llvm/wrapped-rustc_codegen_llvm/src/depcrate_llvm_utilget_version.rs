// Generated macro for get_version (function)
macro_rules! Depcrate_llvm_utilget_version {
() => {
// Module: crate::llvm_util
// Provides: {"get_version"}
// Dependencies: {}
pub (crate) fn get_version () -> (u32 , u32 , u32) { unsafe { (llvm :: LLVMRustVersionMajor () , llvm :: LLVMRustVersionMinor () , llvm :: LLVMRustVersionPatch ()) } }
};
}
