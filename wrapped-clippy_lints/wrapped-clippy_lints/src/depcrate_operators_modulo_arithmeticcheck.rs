// Generated macro for check (function)
macro_rules! Depcrate_operators_modulo_arithmeticcheck {
() => {
// Module: crate::operators::modulo_arithmetic
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , op : BinOpKind , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > , allow_comparison_to_zero : bool ,) { if op == BinOpKind :: Rem { if allow_comparison_to_zero && used_in_comparison_with_zero (cx , e) { return ; } let lhs_operand = analyze_operand (lhs , cx , e) ; let rhs_operand = analyze_operand (rhs , cx , e) ; if let Some (lhs_operand) = lhs_operand && let Some (rhs_operand) = rhs_operand { check_const_operands (cx , e , & lhs_operand , & rhs_operand) ; } else { check_non_const_operands (cx , e , lhs) ; } } }
};
}
