// Generated macro for is_and_minus_one (function)
macro_rules! Depcrate_manual_is_power_of_twois_and_minus_one {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"is_and_minus_one"}
// Dependencies: {}
# [doc = " Return `v` if `expr` is `v & (v - 1)` or `(v - 1) & v`"] fn is_and_minus_one < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { let (lhs , rhs) = unexpanded_binop_operands (expr , BinOpKind :: BitAnd) ? ; is_one_less (cx , lhs , rhs) . or_else (| | is_one_less (cx , rhs , lhs)) }
};
}
