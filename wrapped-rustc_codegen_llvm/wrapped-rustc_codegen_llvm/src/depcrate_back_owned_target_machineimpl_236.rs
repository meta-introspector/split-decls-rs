// Generated macro for impl_236 (impl)
macro_rules! Depcrate_back_owned_target_machineimpl_236 {
() => {
// Module: crate::back::owned_target_machine
// Provides: {"impl_236"}
// Dependencies: {}
impl Drop for OwnedTargetMachine { fn drop (& mut self) { unsafe { llvm :: LLVMRustDisposeTargetMachine (self . tm_unique . as_ptr ()) ; } } }
};
}
