// Generated macro for check (function)
macro_rules! Depcrate_operators_eq_opcheck {
() => {
// Module: crate::operators::eq_op
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > ,) { if is_useless_with_eq_exprs (op) && eq_expr_value (cx , left , right) && ! is_in_test_function (cx . tcx , e . hir_id) { span_lint_and_then (cx , EQ_OP , e . span , format ! ("equal expressions as operands to `{}`" , op . as_str ()) , | diag | { if let BinOpKind :: Ne = op && cx . typeck_results () . expr_ty (left) . is_floating_point () { diag . note ("if you intended to check if the operand is NaN, use `.is_nan()` instead") ; } } ,) ; } }
};
}
