// Generated macro for check (function)
macro_rules! Depcrate_methods_collapsible_str_replacecheck {
() => {
// Module: crate::methods::collapsible_str_replace
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , from : & 'tcx hir :: Expr < 'tcx > , to : & 'tcx hir :: Expr < 'tcx > ,) { let replace_methods = collect_replace_calls (cx , expr , to) ; if replace_methods . methods . len () > 1 { let from_kind = cx . typeck_results () . expr_ty (from) . peel_refs () . kind () ; if let Some (parent) = get_parent_expr (cx , expr) && let Some ((sym :: replace , _ , [current_from , current_to] , _ , _)) = method_call (parent) && eq_expr_value (cx , to , current_to) && from_kind == cx . typeck_results () . expr_ty (current_from) . peel_refs () . kind () { return ; } check_consecutive_replace_calls (cx , expr , & replace_methods , to) ; } }
};
}
