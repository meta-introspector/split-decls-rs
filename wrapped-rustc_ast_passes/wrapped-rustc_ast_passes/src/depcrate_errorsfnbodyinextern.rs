// Generated macro for FnBodyInExtern (struct)
macro_rules! Depcrate_errorsFnBodyInExtern {
() => {
// Module: crate::errors
// Provides: {"FnBodyInExtern"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_fn_body_extern)] # [help] # [note (ast_passes_extern_keyword_link)] pub (crate) struct FnBodyInExtern { # [primary_span] # [label (ast_passes_cannot_have)] pub span : Span , # [suggestion (code = ";" , applicability = "maybe-incorrect")] pub body : Span , # [label] pub block : Span , }
};
}
