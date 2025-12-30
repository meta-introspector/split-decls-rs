// Generated macro for check (function)
macro_rules! Depcrate_transmute_transmuting_nullcheck {
() => {
// Module: crate::transmute::transmuting_null
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , arg : & 'tcx Expr < '_ > , to_ty : Ty < 'tcx >) -> bool { if ! to_ty . is_ref () { return false ; } if let ExprKind :: Path (ref _qpath) = arg . kind && let Some (Constant :: RawPtr (0)) = ConstEvalCtxt :: new (cx) . eval (arg) { span_lint (cx , TRANSMUTING_NULL , expr . span , LINT_MSG) ; return true ; } if let ExprKind :: Cast (inner_expr , _cast_ty) = arg . kind && is_integer_literal (inner_expr , 0) { span_lint (cx , TRANSMUTING_NULL , expr . span , LINT_MSG) ; return true ; } if let ExprKind :: Call (func1 , []) = arg . kind && func1 . basic_res () . is_diag_item (cx , sym :: ptr_null) { span_lint (cx , TRANSMUTING_NULL , expr . span , LINT_MSG) ; return true ; } false }
};
}
