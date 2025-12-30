// Generated macro for check_const_operands (function)
macro_rules! Depcrate_operators_modulo_arithmeticcheck_const_operands {
() => {
// Module: crate::operators::modulo_arithmetic
// Provides: {"check_const_operands"}
// Dependencies: {}
fn check_const_operands < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , lhs_operand : & OperandInfo , rhs_operand : & OperandInfo ,) { if lhs_operand . is_negative ^ rhs_operand . is_negative { span_lint_and_then (cx , MODULO_ARITHMETIC , expr . span , format ! ("you are using modulo operator on constants with different signs: `{} % {}`" , lhs_operand . string_representation . as_ref () . unwrap () , rhs_operand . string_representation . as_ref () . unwrap ()) , | diag | { diag . note ("double check for expected result especially when interoperating with different languages") ; if lhs_operand . is_integral { diag . note ("or consider using `rem_euclid` or similar function") ; } } ,) ; } }
};
}
