// Generated macro for check (function)
macro_rules! Depcrate_methods_suspicious_mapcheck {
() => {
// Module: crate::methods::suspicious_map
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , count_recv : & hir :: Expr < '_ > , map_arg : & hir :: Expr < '_ >) { if cx . ty_based_def (count_recv) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let hir :: ExprKind :: Closure (closure) = expr_or_init (cx , map_arg) . kind && let closure_body = cx . tcx . hir_body (closure . body) && ! cx . typeck_results () . expr_ty (closure_body . value) . is_unit () { if let Some (map_mutated_vars) = mutated_variables (closure_body . value , cx) && ! map_mutated_vars . is_empty () { return ; } span_lint_and_help (cx , SUSPICIOUS_MAP , expr . span , "this call to `map()` won't have an effect on the call to `count()`" , None , "make sure you did not confuse `map` with `filter`, `for_each` or `inspect`" ,) ; } }
};
}
