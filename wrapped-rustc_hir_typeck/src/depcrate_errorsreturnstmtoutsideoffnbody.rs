// Generated macro for ReturnStmtOutsideOfFnBody (struct)
macro_rules! Depcrate_errorsReturnStmtOutsideOfFnBody {
() => {
// Module: crate::errors
// Provides: {"ReturnStmtOutsideOfFnBody"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_return_stmt_outside_of_fn_body , code = E0572)] pub (crate) struct ReturnStmtOutsideOfFnBody { # [primary_span] pub span : Span , # [label (hir_typeck_encl_body_label)] pub encl_body_span : Option < Span > , # [label (hir_typeck_encl_fn_label)] pub encl_fn_span : Option < Span > , pub statement_kind : ReturnLikeStatementKind , }
};
}
