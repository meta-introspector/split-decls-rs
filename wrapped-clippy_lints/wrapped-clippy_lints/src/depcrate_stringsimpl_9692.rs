// Generated macro for impl_9692 (impl)
macro_rules! Depcrate_stringsimpl_9692 {
() => {
// Module: crate::strings
// Provides: {"impl_9692"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for TrimSplitWhitespace { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < '_ >) { let tyckres = cx . typeck_results () ; if let ExprKind :: MethodCall (path , split_recv , [] , split_ws_span) = expr . kind && path . ident . name == sym :: split_whitespace && let Some (split_ws_def_id) = tyckres . type_dependent_def_id (expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: str_split_whitespace , split_ws_def_id) && let ExprKind :: MethodCall (path , _trim_recv , [] , trim_span) = split_recv . kind && let trim_fn_name @ (sym :: trim | sym :: trim_start | sym :: trim_end) = path . ident . name && let Some (trim_def_id) = tyckres . type_dependent_def_id (split_recv . hir_id) && is_one_of_trim_diagnostic_items (cx , trim_def_id) { span_lint_and_sugg (cx , TRIM_SPLIT_WHITESPACE , trim_span . with_hi (split_ws_span . lo ()) , format ! ("found call to `str::{trim_fn_name}` before `str::split_whitespace`") , format ! ("remove `{trim_fn_name}()`") , String :: new () , Applicability :: MachineApplicable ,) ; } } }
};
}
