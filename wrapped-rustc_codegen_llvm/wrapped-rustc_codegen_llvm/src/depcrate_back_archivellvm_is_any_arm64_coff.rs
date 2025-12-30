// Generated macro for llvm_is_any_arm64_coff (function)
macro_rules! Depcrate_back_archivellvm_is_any_arm64_coff {
() => {
// Module: crate::back::archive
// Provides: {"llvm_is_any_arm64_coff"}
// Dependencies: {}
fn llvm_is_any_arm64_coff (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIsAnyArm64Coff (buf . as_ptr () , buf . len ()) } }
};
}
