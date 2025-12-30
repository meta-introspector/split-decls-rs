// Generated macro for llvm_is_ec_object_file (function)
macro_rules! Depcrate_back_archivellvm_is_ec_object_file {
() => {
// Module: crate::back::archive
// Provides: {"llvm_is_ec_object_file"}
// Dependencies: {}
fn llvm_is_ec_object_file (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIsECObject (buf . as_ptr () , buf . len ()) } }
};
}
