// Generated macro for impl_8832 (impl)
macro_rules! Depcrate_ptr_offset_with_castimpl_8832 {
() => {
// Module: crate::ptr_offset_with_cast
// Provides: {"impl_8832"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PtrOffsetWithCast { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let Some ((receiver_expr , arg_expr , method)) = expr_as_ptr_offset_call (cx , expr) else { return ; } ; let Some (cast_lhs_expr) = expr_as_cast_from_usize (cx , arg_expr) else { return ; } ; let msg = format ! ("use of `{method}` with a `usize` casted to an `isize`") ; if let Some (sugg) = build_suggestion (cx , method , receiver_expr , cast_lhs_expr) { span_lint_and_sugg (cx , PTR_OFFSET_WITH_CAST , expr . span , msg , "try" , sugg , Applicability :: MachineApplicable ,) ; } else { span_lint (cx , PTR_OFFSET_WITH_CAST , expr . span , msg) ; } } }
};
}
