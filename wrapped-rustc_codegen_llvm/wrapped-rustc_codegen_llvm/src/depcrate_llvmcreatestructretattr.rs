// Generated macro for CreateStructRetAttr (function)
macro_rules! Depcrate_llvmCreateStructRetAttr {
() => {
// Module: crate::llvm
// Provides: {"CreateStructRetAttr"}
// Dependencies: {}
pub (crate) fn CreateStructRetAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateStructRetAttr (llcx , ty) } }
};
}
