// Generated macro for syntax_diagnostics (function)
macro_rules! Depcratesyntax_diagnostics {
() => {
// Module: crate
// Provides: {"syntax_diagnostics"}
// Dependencies: {}
# [doc = " Request parser level diagnostics for the given [`FileId`]."] pub fn syntax_diagnostics (db : & RootDatabase , config : & DiagnosticsConfig , file_id : FileId ,) -> Vec < Diagnostic > { let _p = tracing :: info_span ! ("syntax_diagnostics") . entered () ; if config . disabled . contains ("syntax-error") { return Vec :: new () ; } let sema = Semantics :: new (db) ; let editioned_file_id = sema . attach_first_edition (file_id) . unwrap_or_else (| | EditionedFileId :: current_edition (db , file_id)) ; let (file_id , _) = editioned_file_id . unpack (db) ; db . parse_errors (editioned_file_id) . into_iter () . flatten () . take (128) . map (| err | { Diagnostic :: new (DiagnosticCode :: SyntaxError , format ! ("Syntax Error: {err}") , FileRange { file_id , range : err . range () } ,) }) . collect () }
};
}
