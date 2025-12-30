// Generated macro for set_dso_local (function)
macro_rules! Depcrate_llvmset_dso_local {
() => {
// Module: crate::llvm
// Provides: {"set_dso_local"}
// Dependencies: {}
pub (crate) fn set_dso_local < 'll > (v : & 'll Value) { unsafe { LLVMRustSetDSOLocal (v , true) ; } }
};
}
