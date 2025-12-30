// Generated macro for DiagnosticSpan (struct)
macro_rules! Depcrate_executor_jsonDiagnosticSpan {
() => {
// Module: crate::executor::json
// Provides: {"DiagnosticSpan"}
// Dependencies: {}
# [derive (Deserialize , Clone)] struct DiagnosticSpan { file_name : String , line_start : usize , column_start : usize , is_primary : bool , label : Option < String > , suggested_replacement : Option < String > , expansion : Option < Box < DiagnosticSpanMacroExpansion > > , }
};
}
