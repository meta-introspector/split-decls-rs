// Generated macro for parse_shift (function)
macro_rules! Depcrate_manual_rotateparse_shift {
() => {
// Module: crate::manual_rotate
// Provides: {"parse_shift"}
// Dependencies: {}
fn parse_shift < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> Option < (ShiftDirection , & 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx >) > { if let ExprKind :: Binary (op , l , r) = expr . kind { let dir = match op . node { BinOpKind :: Shl => ShiftDirection :: Left , BinOpKind :: Shr => ShiftDirection :: Right , _ => return None , } ; return Some ((dir , l , r)) ; } None }
};
}
