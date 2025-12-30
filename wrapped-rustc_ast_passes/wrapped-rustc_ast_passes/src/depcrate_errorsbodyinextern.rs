// Generated macro for BodyInExtern (struct)
macro_rules! Depcrate_errorsBodyInExtern {
() => {
// Module: crate::errors
// Provides: {"BodyInExtern"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_body_in_extern)] # [note (ast_passes_extern_keyword_link)] pub (crate) struct BodyInExtern < 'a > { # [primary_span] # [label (ast_passes_cannot_have)] pub span : Span , # [label (ast_passes_invalid)] pub body : Span , # [label (ast_passes_existing)] pub block : Span , pub kind : & 'a str , }
};
}
