// Generated macro for CreateAllocKindAttr (function)
macro_rules! Depcrate_llvmCreateAllocKindAttr {
() => {
// Module: crate::llvm
// Provides: {"CreateAllocKindAttr"}
// Dependencies: {}
pub (crate) fn CreateAllocKindAttr (llcx : & Context , kind_arg : AllocKindFlags) -> & Attribute { unsafe { LLVMRustCreateAllocKindAttr (llcx , kind_arg . bits ()) } }
};
}
