// Generated macro for AssocFnWithoutBody (struct)
macro_rules! Depcrate_errorsAssocFnWithoutBody {
() => {
// Module: crate::errors
// Provides: {"AssocFnWithoutBody"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_assoc_fn_without_body)] pub (crate) struct AssocFnWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " {{ <body> }}" , applicability = "has-placeholders")] pub replace_span : Span , }
};
}
