// Generated macro for check (function)
macro_rules! Depcrate_methods_ptr_offset_with_castcheck {
() => {
// Module: crate::methods::ptr_offset_with_cast
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , method : Symbol , expr : & Expr < '_ > , recv : & Expr < '_ > , arg : & Expr < '_ > , msrv : Msrv ,) { if ! msrv . meets (cx , msrvs :: POINTER_ADD_SUB_METHODS) { return ; } let method = match method { sym :: offset => Method :: Offset , sym :: wrapping_offset => Method :: WrappingOffset , _ => return , } ; if ! cx . typeck_results () . expr_ty_adjusted (recv) . is_raw_ptr () { return ; } let cast_lhs_expr = match arg . kind { ExprKind :: Cast (lhs , _) if cx . typeck_results () . expr_ty (lhs) . is_usize () => lhs , _ => return , } ; let ExprKind :: MethodCall (method_name , _ , _ , _) = expr . kind else { return ; } ; let msg = format ! ("use of `{method}` with a `usize` casted to an `isize`") ; span_lint_and_then (cx , PTR_OFFSET_WITH_CAST , expr . span , msg , | diag | { diag . multipart_suggestion (format ! ("use `{}` instead" , method . suggestion ()) , vec ! [(method_name . ident . span , method . suggestion () . to_string ()) , (arg . span . with_lo (cast_lhs_expr . span . hi ()) , String :: new ()) ,] , Applicability :: MachineApplicable ,) ; }) ; }
};
}
