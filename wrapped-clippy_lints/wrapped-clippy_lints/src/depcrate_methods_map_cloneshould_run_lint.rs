// Generated macro for should_run_lint (function)
macro_rules! Depcrate_methods_map_cloneshould_run_lint {
() => {
// Module: crate::methods::map_clone
// Provides: {"should_run_lint"}
// Dependencies: {}
fn should_run_lint (cx : & LateContext < '_ > , e : & hir :: Expr < '_ > , method_parent_id : DefId) -> bool { if method_parent_id . is_diag_item (cx , sym :: Iterator) { return true ; } if let Some (ty) = method_parent_id . opt_impl_ty (cx) { if ! ty . is_diag_item (cx , sym :: Option) && ! ty . is_diag_item (cx , sym :: Result) { return false ; } } else { return false ; } if let hir :: ExprKind :: MethodCall (path1 , receiver , _ , _) = & e . kind && let hir :: ExprKind :: MethodCall (path2 , _ , _ , _) = & receiver . kind { return path2 . ident . name != sym :: as_ref || path1 . ident . name != sym :: map ; } true }
};
}
