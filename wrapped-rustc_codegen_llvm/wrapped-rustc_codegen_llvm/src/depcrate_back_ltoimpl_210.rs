// Generated macro for impl_210 (impl)
macro_rules! Depcrate_back_ltoimpl_210 {
() => {
// Module: crate::back::lto
// Provides: {"impl_210"}
// Dependencies: {}
impl Drop for ModuleBuffer { fn drop (& mut self) { unsafe { llvm :: LLVMRustModuleBufferFree (& mut * (self . 0 as * mut _)) ; } } }
};
}
