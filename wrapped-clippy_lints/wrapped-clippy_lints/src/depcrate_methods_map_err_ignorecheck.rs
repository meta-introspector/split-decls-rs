// Generated macro for check (function)
macro_rules! Depcrate_methods_map_err_ignorecheck {
() => {
// Module: crate::methods::map_err_ignore
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , e : & Expr < '_ > , arg : & Expr < '_ >) { if let Some (method_id) = cx . typeck_results () . type_dependent_def_id (e . hir_id) && let Some (impl_id) = cx . tcx . impl_of_assoc (method_id) && cx . tcx . type_of (impl_id) . instantiate_identity () . is_diag_item (cx , sym :: Result) && let ExprKind :: Closure (& Closure { capture_clause : CaptureBy :: Ref , body , fn_decl_span , .. }) = arg . kind && let closure_body = cx . tcx . hir_body (body) && let [param] = closure_body . params && let PatKind :: Wild = param . pat . kind { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , MAP_ERR_IGNORE , fn_decl_span , "`map_err(|_|...` wildcard pattern discards the original error" , | diag | { diag . help ("consider storing the original error as a source in the new error, or silence this warning using an ignored identifier (`.map_err(|_foo| ...`)" ,) ; } ,) ; } }
};
}
