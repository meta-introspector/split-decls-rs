// Generated macro for parent_is_map (function)
macro_rules! Depcrate_methods_iter_filterparent_is_map {
() => {
// Module: crate::methods::iter_filter
// Provides: {"parent_is_map"}
// Dependencies: {}
fn parent_is_map (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { if let Some (expr) = get_parent_expr (cx , expr) && let ExprKind :: MethodCall (path , _ , [_] , _) = expr . kind && path . ident . name == sym :: map && cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { return true ; } false }
};
}
