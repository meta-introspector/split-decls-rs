// Generated macro for lint_binary_expr_with_method_call (function)
macro_rules! Depcrate_methodslint_binary_expr_with_method_call {
() => {
// Module: crate::methods
// Provides: {"lint_binary_expr_with_method_call"}
// Dependencies: {}
# [doc = " Checks for the `CHARS_NEXT_CMP` and `CHARS_LAST_CMP` lints."] fn lint_binary_expr_with_method_call (cx : & LateContext < '_ > , info : & mut BinaryExprInfo < '_ >) { macro_rules ! lint_with_both_lhs_and_rhs { ($ func : expr , $ cx : expr , $ info : ident) => { if !$ func ($ cx , $ info) { :: std :: mem :: swap (& mut $ info . chain , & mut $ info . other) ; if $ func ($ cx , $ info) { return ; } } } ; } lint_with_both_lhs_and_rhs ! (chars_next_cmp :: check , cx , info) ; lint_with_both_lhs_and_rhs ! (chars_last_cmp :: check , cx , info) ; lint_with_both_lhs_and_rhs ! (chars_next_cmp_with_unwrap :: check , cx , info) ; lint_with_both_lhs_and_rhs ! (chars_last_cmp_with_unwrap :: check , cx , info) ; }
};
}
