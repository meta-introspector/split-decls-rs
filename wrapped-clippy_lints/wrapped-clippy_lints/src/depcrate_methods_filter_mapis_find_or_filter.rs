// Generated macro for is_find_or_filter (function)
macro_rules! Depcrate_methods_filter_mapis_find_or_filter {
() => {
// Module: crate::methods::filter_map
// Provides: {"is_find_or_filter"}
// Dependencies: {}
fn is_find_or_filter < 'a > (cx : & LateContext < 'a > , map_recv : & Expr < '_ > , filter_arg : & Expr < '_ > , map_arg : & Expr < '_ > ,) -> Option < (Ident , CheckResult < 'a >) > { if cx . ty_based_def (map_recv) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let ExprKind :: Closure (& Closure { body : filter_body_id , .. }) = filter_arg . kind && let filter_body = cx . tcx . hir_body (filter_body_id) && let [filter_param] = filter_body . params && let (filter_pat , is_filter_param_ref) = if let PatKind :: Ref (ref_pat , _ , _) = filter_param . pat . kind { (ref_pat , true) } else { (filter_param . pat , false) } && let PatKind :: Binding (_ , filter_param_id , _ , None) = filter_pat . kind && let Some (offending_expr) = OffendingFilterExpr :: hir (cx , filter_body . value , filter_param_id) && let ExprKind :: Closure (& Closure { body : map_body_id , .. }) = map_arg . kind && let map_body = cx . tcx . hir_body (map_body_id) && let [map_param] = map_body . params && let PatKind :: Binding (_ , map_param_id , map_param_ident , None) = map_param . pat . kind && let Some (check_result) = offending_expr . check_map_call (cx , map_body , map_param_id , filter_param_id , is_filter_param_ref) { return Some ((map_param_ident , check_result)) ; } None }
};
}
