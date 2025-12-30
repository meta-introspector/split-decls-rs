// Generated macro for simplify (function)
macro_rules! Depcrate_manual_slice_size_calculationsimplify {
() => {
// Module: crate::manual_slice_size_calculation
// Provides: {"simplify"}
// Dependencies: {}
fn simplify < 'tcx > (cx : & LateContext < 'tcx > , expr1 : & 'tcx Expr < 'tcx > , expr2 : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , usize) > { let expr1 = expr_or_init (cx , expr1) ; let expr2 = expr_or_init (cx , expr2) ; simplify_half (cx , expr1 , expr2) . or_else (| | simplify_half (cx , expr2 , expr1)) }
};
}
