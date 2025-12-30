// Generated macro for check (function)
macro_rules! Depcrate_transmute_transmute_null_to_fncheck {
() => {
// Module: crate::transmute::transmute_null_to_fn
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , arg : & 'tcx Expr < '_ > , to_ty : Ty < 'tcx >) -> bool { if ! to_ty . is_fn () { return false ; } let casts_peeled = peel_casts (arg) ; match casts_peeled . kind { ExprKind :: Path (ref _qpath) if matches ! (ConstEvalCtxt :: new (cx) . eval (casts_peeled) , Some (Constant :: RawPtr (0))) => { lint_expr (cx , expr) ; true } , ExprKind :: Call (func1 , []) if func1 . basic_res () . is_diag_item (cx , sym :: ptr_null) => { lint_expr (cx , expr) ; true } , _ => { if is_integer_literal (casts_peeled , 0) { lint_expr (cx , expr) ; return true ; } false } , } }
};
}
