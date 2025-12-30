// Generated macro for check (function)
macro_rules! Depcrate_ptr_ptr_eqcheck {
() => {
// Module: crate::ptr::ptr_eq
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > , span : Span ,) { if span . from_expansion () { return ; } let (left , right , usize_peeled) = match (expr_as_cast_to_usize (cx , left) , expr_as_cast_to_usize (cx , right)) { (Some (lhs) , Some (rhs)) => (lhs , rhs , true) , _ => (left , right , false) , } ; let (left_ty , right_ty) = (cx . typeck_results () . expr_ty (left) , cx . typeck_results () . expr_ty (right)) ; if ! left_ty . is_raw_ptr () || ! right_ty . is_raw_ptr () { return ; } let ((left_var , left_casts_peeled) , (right_var , right_casts_peeled)) = (peel_raw_casts (cx , left , left_ty) , peel_raw_casts (cx , right , right_ty)) ; if ! (usize_peeled || left_casts_peeled || right_casts_peeled) { return ; } let mut app = Applicability :: MachineApplicable ; let ctxt = span . ctxt () ; let left_snip = Sugg :: hir_with_context (cx , left_var , ctxt , "_" , & mut app) ; let right_snip = Sugg :: hir_with_context (cx , right_var , ctxt , "_" , & mut app) ; { let Some (top_crate) = std_or_core (cx) else { return } ; let invert = if op == BinOpKind :: Eq { "" } else { "!" } ; span_lint_and_sugg (cx , PTR_EQ , span , format ! ("use `{top_crate}::ptr::eq` when comparing raw pointers") , "try" , format ! ("{invert}{top_crate}::ptr::eq({left_snip}, {right_snip})") , app ,) ; } }
};
}
