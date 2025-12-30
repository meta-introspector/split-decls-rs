// Generated macro for check (function)
macro_rules! Depcrate_operators_misrefactored_assign_opcheck {
() => {
// Module: crate::operators::misrefactored_assign_op
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , op : hir :: BinOpKind , lhs : & 'tcx hir :: Expr < '_ > , rhs : & 'tcx hir :: Expr < '_ > ,) { if let hir :: ExprKind :: Binary (binop , l , r) = & rhs . kind { if op != binop . node { return ; } if eq_expr_value (cx , lhs , l) { lint_misrefactored_assign_op (cx , expr , op , rhs , lhs , r) ; } else if is_commutative (op) && eq_expr_value (cx , lhs , r) { lint_misrefactored_assign_op (cx , expr , op , rhs , lhs , l) ; } } }
};
}
