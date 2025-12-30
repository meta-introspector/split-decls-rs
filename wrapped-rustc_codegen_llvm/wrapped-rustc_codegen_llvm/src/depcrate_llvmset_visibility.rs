// Generated macro for set_visibility (function)
macro_rules! Depcrate_llvmset_visibility {
() => {
// Module: crate::llvm
// Provides: {"set_visibility"}
// Dependencies: {}
pub (crate) fn set_visibility (llglobal : & Value , visibility : Visibility) { unsafe { LLVMSetVisibility (llglobal , visibility) ; } }
};
}
