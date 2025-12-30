// Generated macro for DiagnosticsTaskKind (enum)
macro_rules! Depcrate_main_loopDiagnosticsTaskKind {
() => {
// Module: crate::main_loop
// Provides: {"DiagnosticsTaskKind"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum DiagnosticsTaskKind { Syntax (DiagnosticsGeneration , Vec < (FileId , Vec < lsp_types :: Diagnostic >) >) , Semantic (DiagnosticsGeneration , Vec < (FileId , Vec < lsp_types :: Diagnostic >) >) , }
};
}
