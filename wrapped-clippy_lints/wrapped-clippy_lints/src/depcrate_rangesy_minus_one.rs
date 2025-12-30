// Generated macro for y_minus_one (function)
macro_rules! Depcrate_rangesy_minus_one {
() => {
// Module: crate::ranges
// Provides: {"y_minus_one"}
// Dependencies: {}
fn y_minus_one < 'tcx > (cx : & LateContext < '_ > , expr : & Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { match expr . kind { ExprKind :: Binary (Spanned { node : BinOpKind :: Sub , .. } , lhs , rhs ,) if is_integer_const (cx , rhs , 1) => Some (lhs) , _ => None , } }
};
}
