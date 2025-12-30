// Generated macro for set_initializer (function)
macro_rules! Depcrate_llvmset_initializer {
() => {
// Module: crate::llvm
// Provides: {"set_initializer"}
// Dependencies: {}
pub (crate) fn set_initializer (llglobal : & Value , constant_val : & Value) { unsafe { LLVMSetInitializer (llglobal , constant_val) ; } }
};
}
