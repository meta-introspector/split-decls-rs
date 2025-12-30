// Generated macro for DiagnosticHandlers (struct)
macro_rules! Depcrate_back_writeDiagnosticHandlers {
() => {
// Module: crate::back::write
// Provides: {"DiagnosticHandlers"}
// Dependencies: {}
pub (crate) struct DiagnosticHandlers < 'a > { data : * mut (& 'a CodegenContext < LlvmCodegenBackend > , DiagCtxtHandle < 'a >) , llcx : & 'a llvm :: Context , old_handler : Option < & 'a llvm :: DiagnosticHandler > , }
};
}
