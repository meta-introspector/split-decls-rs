// Generated macro for impl_1249 (impl)
macro_rules! Depcrateimpl_1249 {
() => {
// Module: crate
// Provides: {"impl_1249"}
// Dependencies: {}
impl Drop for ModuleLlvm { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . tm) ; llvm :: LLVMContextDispose (& mut * (self . llcx as * mut _)) ; } } }
};
}
