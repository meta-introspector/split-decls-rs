// Generated macro for unit_closure (function)
macro_rules! Depcrate_map_unit_fnunit_closure {
() => {
// Module: crate::map_unit_fn
// Provides: {"unit_closure"}
// Dependencies: {}
fn unit_closure < 'tcx > (cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ > ,) -> Option < (& 'tcx hir :: Param < 'tcx > , & 'tcx hir :: Expr < 'tcx >) > { if let hir :: ExprKind :: Closure (& hir :: Closure { fn_decl , body , .. }) = expr . kind && let body = cx . tcx . hir_body (body) && let body_expr = & body . value && fn_decl . inputs . len () == 1 && is_unit_expression (cx , body_expr) && let Some (binding) = iter_input_pats (fn_decl , body) . next () { return Some ((binding , body_expr)) ; } None }
};
}
