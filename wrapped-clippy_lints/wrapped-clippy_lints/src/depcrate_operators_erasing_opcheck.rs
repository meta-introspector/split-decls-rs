// Generated macro for check (function)
macro_rules! Depcrate_operators_erasing_opcheck {
() => {
// Module: crate::operators::erasing_op
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > ,) { let tck = cx . typeck_results () ; match op { BinOpKind :: Mul | BinOpKind :: BitAnd => { check_op (cx , tck , left , right , e) ; check_op (cx , tck , right , left , e) ; } , BinOpKind :: Div => check_op (cx , tck , left , right , e) , _ => () , } }
};
}
