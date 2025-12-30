// Generated macro for check (function)
macro_rules! Depcrate_operators_manual_midpointcheck {
() => {
// Module: crate::operators::manual_midpoint
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > , msrv : Msrv ,) { if ! left . span . from_expansion () && ! right . span . from_expansion () && op == BinOpKind :: Div && (is_integer_literal (right , 2) || is_float_literal (right , 2.0)) && let Some ((ll_expr , lr_expr)) = add_operands (left) && add_operands (ll_expr) . is_none () && add_operands (lr_expr) . is_none () && let left_ty = cx . typeck_results () . expr_ty_adjusted (ll_expr) && let right_ty = cx . typeck_results () . expr_ty_adjusted (lr_expr) && left_ty == right_ty && ! is_integer_literal (ll_expr , 1) && ! is_integer_literal (lr_expr , 1) && is_midpoint_implemented (cx , left_ty , msrv) { let mut app = Applicability :: MachineApplicable ; let left_sugg = Sugg :: hir_with_context (cx , ll_expr , expr . span . ctxt () , ".." , & mut app) ; let right_sugg = Sugg :: hir_with_context (cx , lr_expr , expr . span . ctxt () , ".." , & mut app) ; let sugg = format ! ("{left_ty}::midpoint({left_sugg}, {right_sugg})") ; span_lint_and_sugg (cx , MANUAL_MIDPOINT , expr . span , "manual implementation of `midpoint` which can overflow" , format ! ("use `{left_ty}::midpoint` instead") , sugg , app ,) ; } }
};
}
