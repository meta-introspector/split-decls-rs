// Generated macro for mutex_lock_call (function)
macro_rules! Depcrate_if_let_mutexmutex_lock_call {
() => {
// Module: crate::if_let_mutex
// Provides: {"mutex_lock_call"}
// Dependencies: {}
fn mutex_lock_call < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op_mutex : Option < & 'tcx Expr < '_ > > ,) -> ControlFlow < & 'tcx Expr < 'tcx > > { if let ExprKind :: MethodCall (path , self_arg , [] , _) = & expr . kind && path . ident . name == sym :: lock && let ty = cx . typeck_results () . expr_ty (self_arg) . peel_refs () && ty . is_diag_item (cx , sym :: Mutex) && op_mutex . is_none_or (| op | eq_expr_value (cx , self_arg , op)) { ControlFlow :: Break (self_arg) } else { ControlFlow :: Continue (()) } }
};
}
