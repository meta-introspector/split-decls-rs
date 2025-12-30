// Generated macro for get_const (function)
macro_rules! Depcrate_implicit_saturating_addget_const {
() => {
// Module: crate::implicit_saturating_add
// Provides: {"get_const"}
// Dependencies: {}
fn get_const < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < (u128 , BinOpKind , & 'tcx Expr < 'tcx >) > { if let ExprKind :: Binary (op , l , r) = expr . kind { let ecx = ConstEvalCtxt :: new (cx) ; let ctxt = expr . span . ctxt () ; if let Some (Constant :: Int (c)) = ecx . eval_local (r , ctxt) { return Some ((c , op . node , l)) ; } if let Some (Constant :: Int (c)) = ecx . eval_local (l , ctxt) { return Some ((c , invert_op (op . node) ? , r)) ; } } None }
};
}
