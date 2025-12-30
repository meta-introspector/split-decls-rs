// Generated macro for set_global_constant (function)
macro_rules! Depcrate_llvmset_global_constant {
() => {
// Module: crate::llvm
// Provides: {"set_global_constant"}
// Dependencies: {}
pub (crate) fn set_global_constant (llglobal : & Value , is_constant : bool) { LLVMSetGlobalConstant (llglobal , is_constant . to_llvm_bool ()) ; }
};
}
