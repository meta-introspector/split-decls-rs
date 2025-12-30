// Generated macro for uint_compare_to_zero (function)
macro_rules! Depcrate_operators_manual_is_multiple_ofuint_compare_to_zero {
() => {
// Module: crate::operators::manual_is_multiple_of
// Provides: {"uint_compare_to_zero"}
// Dependencies: {}
fn uint_compare_to_zero < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx > , op : BinOpKind , lhs : & 'tcx Expr < 'tcx > , rhs : & 'tcx Expr < 'tcx > ,) -> Option < & 'tcx Expr < 'tcx > > { let operand = if matches ! (lhs . kind , ExprKind :: Binary (..)) && matches ! (op , BinOpKind :: Eq | BinOpKind :: Ne | BinOpKind :: Gt) && is_zero_integer_const (cx , rhs , e . span . ctxt ()) { lhs } else if matches ! (rhs . kind , ExprKind :: Binary (..)) && matches ! (op , BinOpKind :: Eq | BinOpKind :: Ne | BinOpKind :: Lt) && is_zero_integer_const (cx , lhs , e . span . ctxt ()) { rhs } else { return None ; } ; matches ! (cx . typeck_results () . expr_ty_adjusted (operand) . kind () , ty :: Uint (_)) . then_some (operand) }
};
}
