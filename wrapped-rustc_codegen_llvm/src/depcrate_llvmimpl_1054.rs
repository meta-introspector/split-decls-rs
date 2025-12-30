// Generated macro for impl_1054 (impl)
macro_rules! Depcrate_llvmimpl_1054 {
() => {
// Module: crate::llvm
// Provides: {"impl_1054"}
// Dependencies: {}
impl AttributeKind { # [doc = " Create an LLVM Attribute with no associated value."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateAttrNoValue (llcx , self) } } }
};
}
