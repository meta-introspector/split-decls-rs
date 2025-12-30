// Generated macro for CreateAlignmentAttr (function)
macro_rules! Depcrate_llvmCreateAlignmentAttr {
() => {
// Module: crate::llvm
// Provides: {"CreateAlignmentAttr"}
// Dependencies: {}
pub (crate) fn CreateAlignmentAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateAlignmentAttr (llcx , bytes) } }
};
}
