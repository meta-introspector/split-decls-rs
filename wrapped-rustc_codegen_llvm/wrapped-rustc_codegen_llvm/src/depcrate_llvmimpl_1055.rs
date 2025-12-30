// Generated macro for impl_1055 (impl)
macro_rules! Depcrate_llvmimpl_1055 {
() => {
// Module: crate::llvm
// Provides: {"impl_1055"}
// Dependencies: {}
impl MemoryEffects { # [doc = " Create an LLVM Attribute with these memory effects."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateMemoryEffectsAttr (llcx , self) } } }
};
}
