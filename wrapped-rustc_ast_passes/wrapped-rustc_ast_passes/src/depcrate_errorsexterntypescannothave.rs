// Generated macro for ExternTypesCannotHave (struct)
macro_rules! Depcrate_errorsExternTypesCannotHave {
() => {
// Module: crate::errors
// Provides: {"ExternTypesCannotHave"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_extern_types_cannot)] # [note (ast_passes_extern_keyword_link)] pub (crate) struct ExternTypesCannotHave < 'a > { # [primary_span] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , pub descr : & 'a str , pub remove_descr : & 'a str , # [label] pub block_span : Span , }
};
}
