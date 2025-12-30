// Generated macro for are_diagnostics_equal (function)
macro_rules! Depcrate_cli_diagnosticsare_diagnostics_equal {
() => {
// Module: crate::cli::diagnostics
// Provides: {"are_diagnostics_equal"}
// Dependencies: {}
fn are_diagnostics_equal (left : & lsp_types :: Diagnostic , right : & lsp_types :: Diagnostic) -> bool { left . source == right . source && left . severity == right . severity && left . range == right . range && left . message == right . message }
};
}
