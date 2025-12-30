// Generated macro for set_thread_local_mode (function)
macro_rules! Depcrate_llvmset_thread_local_mode {
() => {
// Module: crate::llvm
// Provides: {"set_thread_local_mode"}
// Dependencies: {}
pub (crate) fn set_thread_local_mode (global : & Value , mode : ThreadLocalMode) { unsafe { LLVMSetThreadLocalMode (global , mode) ; } }
};
}
