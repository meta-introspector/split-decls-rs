// Generated macro for check_ptr_eq (function)
macro_rules! Depcrate_ptrcheck_ptr_eq {
() => {
// Module: crate::ptr
// Provides: {"check_ptr_eq"}
// Dependencies: {}
fn check_ptr_eq < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > ,) { if expr . span . from_expansion () { return ; } let (left , right , usize_peeled) = match (expr_as_cast_to_usize (cx , left) , expr_as_cast_to_usize (cx , right)) { (Some (lhs) , Some (rhs)) => (lhs , rhs , true) , _ => (left , right , false) , } ; let (left_ty , right_ty) = (cx . typeck_results () . expr_ty (left) , cx . typeck_results () . expr_ty (right)) ; if ! left_ty . is_raw_ptr () || ! right_ty . is_raw_ptr () { return ; } let ((left_var , left_casts_peeled) , (right_var , right_casts_peeled)) = (peel_raw_casts (cx , left , left_ty) , peel_raw_casts (cx , right , right_ty)) ; if ! (usize_peeled || left_casts_peeled || right_casts_peeled) { return ; } let mut app = Applicability :: MachineApplicable ; let left_snip = Sugg :: hir_with_context (cx , left_var , expr . span . ctxt () , "_" , & mut app) ; let right_snip = Sugg :: hir_with_context (cx , right_var , expr . span . ctxt () , "_" , & mut app) ; { let Some (top_crate) = std_or_core (cx) else { return } ; let invert = if op == BinOpKind :: Eq { "" } else { "!" } ; span_lint_and_sugg (cx , PTR_EQ , expr . span , format ! ("use `{top_crate}::ptr::eq` when comparing raw pointers") , "try" , format ! ("{invert}{top_crate}::ptr::eq({left_snip}, {right_snip})") , app ,) ; } }
};
}
