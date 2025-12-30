// Generated macro for check_op (function)
macro_rules! Depcrate_operators_erasing_opcheck_op {
() => {
// Module: crate::operators::erasing_op
// Provides: {"check_op"}
// Dependencies: {}
fn check_op < 'tcx > (cx : & LateContext < 'tcx > , tck : & 'tcx TypeckResults < 'tcx > , op : & Expr < 'tcx > , other : & Expr < 'tcx > , parent : & Expr < 'tcx > ,) { if ConstEvalCtxt :: with_env (cx . tcx , cx . typing_env () , tck) . eval_local (op , parent . span . ctxt ()) == Some (Constant :: Int (0)) { if different_types (tck , other , parent) { return ; } span_lint (cx , ERASING_OP , parent . span , "this operation will always return zero. This is likely not the intended outcome" ,) ; } }
};
}
