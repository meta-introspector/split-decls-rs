// Generated macro for set_linkage (function)
macro_rules! Depcrate_llvmset_linkage {
() => {
// Module: crate::llvm
// Provides: {"set_linkage"}
// Dependencies: {}
pub (crate) fn set_linkage (llglobal : & Value , linkage : Linkage) { unsafe { LLVMSetLinkage (llglobal , linkage) ; } }
};
}
