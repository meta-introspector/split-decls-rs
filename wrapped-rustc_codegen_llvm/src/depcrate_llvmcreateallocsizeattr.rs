// Generated macro for CreateAllocSizeAttr (function)
macro_rules! Depcrate_llvmCreateAllocSizeAttr {
() => {
// Module: crate::llvm
// Provides: {"CreateAllocSizeAttr"}
// Dependencies: {}
pub (crate) fn CreateAllocSizeAttr (llcx : & Context , size_arg : u32) -> & Attribute { unsafe { LLVMRustCreateAllocSizeAttr (llcx , size_arg) } }
};
}
