// Generated macro for check (function)
macro_rules! Depcrate_methods_result_map_or_else_nonecheck {
() => {
// Module: crate::methods::result_map_or_else_none
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `_.map_or_else(|_| None, Some)` for `Result`s"] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , recv : & 'tcx hir :: Expr < '_ > , def_arg : & 'tcx hir :: Expr < '_ > , map_arg : & 'tcx hir :: Expr < '_ > ,) { if cx . typeck_results () . expr_ty (recv) . is_diag_item (cx , sym :: Result) && map_arg . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionSome) && let hir :: ExprKind :: Closure (& hir :: Closure { body , .. }) = def_arg . kind && let body = cx . tcx . hir_body (body) && is_none_expr (cx , peel_blocks (body . value)) { let msg = "called `map_or_else(|_| None, Some)` on a `Result` value" ; let self_snippet = snippet (cx , recv . span , "..") ; span_lint_and_sugg (cx , RESULT_MAP_OR_INTO_OPTION , expr . span , msg , "consider using `ok`" , format ! ("{self_snippet}.ok()") , Applicability :: MachineApplicable ,) ; } }
};
}
