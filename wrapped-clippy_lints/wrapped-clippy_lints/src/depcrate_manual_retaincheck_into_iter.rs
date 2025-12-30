// Generated macro for check_into_iter (function)
macro_rules! Depcrate_manual_retaincheck_into_iter {
() => {
// Module: crate::manual_retain
// Provides: {"check_into_iter"}
// Dependencies: {}
fn check_into_iter (cx : & LateContext < '_ > , left_expr : & hir :: Expr < '_ > , target_expr : & hir :: Expr < '_ > , parent_expr_span : Span , msrv : Msrv ,) { if let hir :: ExprKind :: MethodCall (_ , into_iter_expr , [_] , _) = & target_expr . kind && let Some (filter_def_id) = cx . typeck_results () . type_dependent_def_id (target_expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: iter_filter , filter_def_id) && let hir :: ExprKind :: MethodCall (_ , struct_expr , [] , _) = & into_iter_expr . kind && let Some (into_iter_def_id) = cx . typeck_results () . type_dependent_def_id (into_iter_expr . hir_id) && Some (into_iter_def_id) == cx . tcx . lang_items () . into_iter_fn () && match_acceptable_type (cx , left_expr , msrv) && SpanlessEq :: new (cx) . eq_expr (left_expr , struct_expr) && let hir :: ExprKind :: MethodCall (_ , _ , [closure_expr] , _) = target_expr . kind && let hir :: ExprKind :: Closure (closure) = closure_expr . kind && let filter_body = cx . tcx . hir_body (closure . body) && let [filter_params] = filter_body . params { if match_map_type (cx , left_expr) { if let hir :: PatKind :: Tuple ([key_pat , value_pat] , _) = filter_params . pat . kind && let Some (sugg) = make_sugg (cx , key_pat , value_pat , left_expr , filter_body) { make_span_lint_and_sugg (cx , parent_expr_span , sugg) ; } } else { make_span_lint_and_sugg (cx , parent_expr_span , format ! ("{}.retain({})" , snippet (cx , left_expr . span , "..") , snippet (cx , closure_expr . span , "..")) ,) ; } } }
};
}
