// Generated macro for get_visibility (function)
macro_rules! Depcrate_llvmget_visibility {
() => {
// Module: crate::llvm
// Provides: {"get_visibility"}
// Dependencies: {}
pub (crate) fn get_visibility (llglobal : & Value) -> Visibility { unsafe { LLVMGetVisibility (llglobal) } . to_rust () }
};
}
