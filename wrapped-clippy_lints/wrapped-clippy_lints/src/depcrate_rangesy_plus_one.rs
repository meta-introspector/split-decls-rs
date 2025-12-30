// Generated macro for y_plus_one (function)
macro_rules! Depcrate_rangesy_plus_one {
() => {
// Module: crate::ranges
// Provides: {"y_plus_one"}
// Dependencies: {}
fn y_plus_one < 'tcx > (cx : & LateContext < '_ > , expr : & Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { match expr . kind { ExprKind :: Binary (Spanned { node : BinOpKind :: Add , .. } , lhs , rhs ,) => { if is_integer_const (cx , lhs , 1) { Some (rhs) } else if is_integer_const (cx , rhs , 1) { Some (lhs) } else { None } } , _ => None , } }
};
}
