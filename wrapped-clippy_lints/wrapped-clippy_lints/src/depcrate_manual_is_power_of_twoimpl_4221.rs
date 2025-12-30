// Generated macro for impl_4221 (impl)
macro_rules! Depcrate_manual_is_power_of_twoimpl_4221 {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"impl_4221"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualIsPowerOfTwo { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if ! expr . span . from_expansion () && let Some ((lhs , rhs)) = unexpanded_binop_operands (expr , BinOpKind :: Eq) { if let Some (a) = count_ones_receiver (cx , lhs) && is_integer_literal (rhs , 1) { self . build_sugg (cx , expr , a) ; } else if let Some (a) = count_ones_receiver (cx , rhs) && is_integer_literal (lhs , 1) { self . build_sugg (cx , expr , a) ; } else if is_integer_literal (rhs , 0) && let Some (a) = is_and_minus_one (cx , lhs) { self . build_sugg (cx , expr , a) ; } else if is_integer_literal (lhs , 0) && let Some (a) = is_and_minus_one (cx , rhs) { self . build_sugg (cx , expr , a) ; } } } }
};
}
