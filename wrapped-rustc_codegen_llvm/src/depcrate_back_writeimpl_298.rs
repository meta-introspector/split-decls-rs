// Generated macro for impl_298 (impl)
macro_rules! Depcrate_back_writeimpl_298 {
() => {
// Module: crate::back::write
// Provides: {"impl_298"}
// Dependencies: {}
impl < 'a > Drop for DiagnosticHandlers < 'a > { fn drop (& mut self) { unsafe { llvm :: LLVMRustContextSetDiagnosticHandler (self . llcx , self . old_handler) ; drop (Box :: from_raw (self . data)) ; } } }
};
}
