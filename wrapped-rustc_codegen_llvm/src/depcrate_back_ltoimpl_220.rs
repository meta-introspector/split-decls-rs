// Generated macro for impl_220 (impl)
macro_rules! Depcrate_back_ltoimpl_220 {
() => {
// Module: crate::back::lto
// Provides: {"impl_220"}
// Dependencies: {}
impl Drop for ThinBuffer { fn drop (& mut self) { unsafe { llvm :: LLVMRustThinLTOBufferFree (& mut * (self . 0 as * mut _)) ; } } }
};
}
