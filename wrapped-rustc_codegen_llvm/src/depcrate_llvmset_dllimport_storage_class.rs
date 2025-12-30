// Generated macro for set_dllimport_storage_class (function)
macro_rules! Depcrate_llvmset_dllimport_storage_class {
() => {
// Module: crate::llvm
// Provides: {"set_dllimport_storage_class"}
// Dependencies: {}
pub (crate) fn set_dllimport_storage_class < 'll > (v : & 'll Value) { unsafe { LLVMSetDLLStorageClass (v , DLLStorageClass :: DllImport) ; } }
};
}
