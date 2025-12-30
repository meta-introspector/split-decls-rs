// Generated macro for check_non_const_operands (function)
macro_rules! Depcrate_operators_modulo_arithmeticcheck_non_const_operands {
() => {
// Module: crate::operators::modulo_arithmetic
// Provides: {"check_non_const_operands"}
// Dependencies: {}
fn check_non_const_operands < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , operand : & Expr < '_ >) { let operand_type = cx . typeck_results () . expr_ty (operand) ; if might_have_negative_value (operand_type) { span_lint_and_then (cx , MODULO_ARITHMETIC , expr . span , "you are using modulo operator on types that might have different signs" , | diag | { diag . note ("double check for expected result especially when interoperating with different languages") ; if operand_type . is_integral () { diag . note ("or consider using `rem_euclid` or similar function") ; } } ,) ; } }
};
}
