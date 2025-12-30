// Generated macro for get_linkage (function)
macro_rules! Depcrate_llvmget_linkage {
() => {
// Module: crate::llvm
// Provides: {"get_linkage"}
// Dependencies: {}
pub (crate) fn get_linkage (llglobal : & Value) -> Linkage { unsafe { LLVMGetLinkage (llglobal) } . to_rust () }
};
}
