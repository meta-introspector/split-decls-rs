// Generated macro for ConstWithoutBody (struct)
macro_rules! Depcrate_errorsConstWithoutBody {
() => {
// Module: crate::errors
// Provides: {"ConstWithoutBody"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_const_without_body)] pub (crate) struct ConstWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
};
}
