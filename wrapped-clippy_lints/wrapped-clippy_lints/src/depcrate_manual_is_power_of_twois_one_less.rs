// Generated macro for is_one_less (function)
macro_rules! Depcrate_manual_is_power_of_twois_one_less {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"is_one_less"}
// Dependencies: {}
# [doc = " Return `greater` if `smaller == greater - 1`"] fn is_one_less < 'tcx > (cx : & LateContext < 'tcx > , greater : & 'tcx Expr < 'tcx > , smaller : & Expr < 'tcx > ,) -> Option < & 'tcx Expr < 'tcx > > { if let Some ((lhs , rhs)) = unexpanded_binop_operands (smaller , BinOpKind :: Sub) && SpanlessEq :: new (cx) . eq_expr (greater , lhs) && is_integer_literal (rhs , 1) && matches ! (cx . typeck_results () . expr_ty_adjusted (greater) . kind () , ty :: Uint (_)) { Some (greater) } else { None } }
};
}
