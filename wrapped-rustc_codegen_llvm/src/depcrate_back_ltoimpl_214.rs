// Generated macro for impl_214 (impl)
macro_rules! Depcrate_back_ltoimpl_214 {
() => {
// Module: crate::back::lto
// Provides: {"impl_214"}
// Dependencies: {}
impl Drop for ThinData { fn drop (& mut self) { unsafe { llvm :: LLVMRustFreeThinLTOData (& mut * (self . 0 as * mut _)) ; } } }
};
}
