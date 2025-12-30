// Generated macro for CreateDereferenceableAttr (function)
macro_rules! Depcrate_llvmCreateDereferenceableAttr {
() => {
// Module: crate::llvm
// Provides: {"CreateDereferenceableAttr"}
// Dependencies: {}
pub (crate) fn CreateDereferenceableAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableAttr (llcx , bytes) } }
};
}
