// Generated macro for DiagnosticCollection (struct)
macro_rules! Depcrate_cli_diagnosticsDiagnosticCollection {
() => {
// Module: crate::cli::diagnostics
// Provides: {"DiagnosticCollection"}
// Dependencies: {}
# [derive (Debug , Default , Clone)] pub (crate) struct DiagnosticCollection { pub (crate) native_syntax : FxHashMap < FileId , (DiagnosticsGeneration , Vec < lsp_types :: Diagnostic >) > , pub (crate) native_semantic : FxHashMap < FileId , (DiagnosticsGeneration , Vec < lsp_types :: Diagnostic >) > , pub (crate) check : Vec < WorkspaceFlycheckDiagnostic > , pub (crate) check_fixes : CheckFixes , changes : FxHashSet < FileId > , # [doc = " Counter for supplying a new generation number for diagnostics."] # [doc = " This is used to keep track of when to clear the diagnostics for a given file as we compute"] # [doc = " diagnostics on multiple worker threads simultaneously which may result in multiple diagnostics"] # [doc = " updates for the same file in a single generation update (due to macros affecting multiple files)."] generation : DiagnosticsGeneration , }
};
}
