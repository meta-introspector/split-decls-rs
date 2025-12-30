// Generated macro for convert_diagnostic (function)
macro_rules! Depcrate_cli_diagnosticsconvert_diagnostic {
() => {
// Module: crate::cli::diagnostics
// Provides: {"convert_diagnostic"}
// Dependencies: {}
pub (crate) fn convert_diagnostic (line_index : & crate :: line_index :: LineIndex , d : ide :: Diagnostic ,) -> lsp_types :: Diagnostic { lsp_types :: Diagnostic { range : lsp :: to_proto :: range (line_index , d . range . range) , severity : Some (lsp :: to_proto :: diagnostic_severity (d . severity)) , code : Some (lsp_types :: NumberOrString :: String (d . code . as_str () . to_owned ())) , code_description : Some (lsp_types :: CodeDescription { href : lsp_types :: Url :: parse (& d . code . url ()) . unwrap () , }) , source : Some ("rust-analyzer" . to_owned ()) , message : d . message , related_information : None , tags : d . unused . then (| | vec ! [lsp_types :: DiagnosticTag :: UNNECESSARY]) , data : None , } }
};
}
