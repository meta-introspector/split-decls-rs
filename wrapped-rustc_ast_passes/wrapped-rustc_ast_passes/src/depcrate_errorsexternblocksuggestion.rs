// Generated macro for ExternBlockSuggestion (enum)
macro_rules! Depcrate_errorsExternBlockSuggestion {
() => {
// Module: crate::errors
// Provides: {"ExternBlockSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ExternBlockSuggestion { # [multipart_suggestion (ast_passes_extern_block_suggestion , applicability = "maybe-incorrect")] Implicit { # [suggestion_part (code = "extern {{")] start_span : Span , # [suggestion_part (code = " }}")] end_span : Span , } , # [multipart_suggestion (ast_passes_extern_block_suggestion , applicability = "maybe-incorrect")] Explicit { # [suggestion_part (code = "extern \"{abi}\" {{")] start_span : Span , # [suggestion_part (code = " }}")] end_span : Span , abi : Symbol , } , }
};
}
