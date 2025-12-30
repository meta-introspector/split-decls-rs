// Generated macro for StaticWithoutBody (struct)
macro_rules! Depcrate_errorsStaticWithoutBody {
() => {
// Module: crate::errors
// Provides: {"StaticWithoutBody"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_static_without_body)] pub (crate) struct StaticWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
};
}
