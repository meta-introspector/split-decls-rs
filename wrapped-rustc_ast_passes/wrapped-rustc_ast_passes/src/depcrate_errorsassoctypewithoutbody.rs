// Generated macro for AssocTypeWithoutBody (struct)
macro_rules! Depcrate_errorsAssocTypeWithoutBody {
() => {
// Module: crate::errors
// Provides: {"AssocTypeWithoutBody"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_assoc_type_without_body)] pub (crate) struct AssocTypeWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <type>;" , applicability = "has-placeholders")] pub replace_span : Span , }
};
}
