// Generated macro for CreateByValAttr (function)
macro_rules! Depcrate_llvmCreateByValAttr {
() => {
// Module: crate::llvm
// Provides: {"CreateByValAttr"}
// Dependencies: {}
pub (crate) fn CreateByValAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateByValAttr (llcx , ty) } }
};
}
