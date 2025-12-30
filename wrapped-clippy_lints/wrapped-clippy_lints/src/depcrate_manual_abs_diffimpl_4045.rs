// Generated macro for impl_4045 (impl)
macro_rules! Depcrate_manual_abs_diffimpl_4045 {
() => {
// Module: crate::manual_abs_diff
// Provides: {"impl_4045"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualAbsDiff { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if ! expr . span . from_expansion () && let Some (if_expr) = If :: hir (expr) && let Some (r#else) = if_expr . r#else && let ExprKind :: Binary (op , rhs , lhs) = if_expr . cond . kind && let (BinOpKind :: Gt | BinOpKind :: Ge , mut a , mut b) | (BinOpKind :: Lt | BinOpKind :: Le , mut b , mut a) = (op . node , rhs , lhs) && let Some ((ty , b_n_refs)) = self . are_ty_eligible (cx , a , b) && is_sub_expr (cx , if_expr . then , a , b , ty) && is_sub_expr (cx , r#else , b , a , ty) { span_lint_and_then (cx , MANUAL_ABS_DIFF , expr . span , "manual absolute difference pattern without using `abs_diff`" , | diag | { if is_unsuffixed_numeral_lit (a) && ! is_unsuffixed_numeral_lit (b) { (a , b) = (b , a) ; } let applicability = { let source_map = cx . sess () . source_map () ; if span_contains_comment (source_map , if_expr . then . span) || span_contains_comment (source_map , r#else . span) { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } } ; let sugg = format ! ("{}.abs_diff({}{})" , Sugg :: hir (cx , a , "..") . maybe_paren () , "*" . repeat (b_n_refs) , Sugg :: hir (cx , b , "..")) ; diag . span_suggestion (expr . span , "replace with `abs_diff`" , sugg , applicability) ; } ,) ; } } }
};
}
