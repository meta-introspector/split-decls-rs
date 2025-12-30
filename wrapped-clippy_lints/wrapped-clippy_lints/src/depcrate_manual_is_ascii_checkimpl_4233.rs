// Generated macro for impl_4233 (impl)
macro_rules! Depcrate_manual_is_ascii_checkimpl_4233 {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"impl_4233"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualIsAsciiCheck { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if ! self . msrv . meets (cx , msrvs :: IS_ASCII_DIGIT) { return ; } if is_in_const_context (cx) && ! self . msrv . meets (cx , msrvs :: IS_ASCII_DIGIT_CONST) { return ; } let (arg , span , range) = if let Some (macro_call) = matching_root_macro_call (cx , expr . span , sym :: matches_macro) && let ExprKind :: Match (recv , [arm , ..] , _) = expr . kind { let recv = peel_ref_operators (cx , recv) ; let range = check_pat (& arm . pat . kind) ; (recv , macro_call . span , range) } else if let ExprKind :: MethodCall (path , receiver , [arg] , ..) = expr . kind && path . ident . name == sym :: contains && let Some (higher :: Range { start : Some (start) , end : Some (end) , limits : RangeLimits :: Closed , span : _ , }) = higher :: Range :: hir (cx , receiver) && ! matches ! (cx . typeck_results () . expr_ty (arg) . peel_refs () . kind () , ty :: Param (_)) { let arg = peel_ref_operators (cx , arg) ; let range = check_expr_range (start , end) ; (arg , expr . span , range) } else { return ; } ; let ty_sugg = get_ty_sugg (cx , arg) ; check_is_ascii (cx , span , arg , & range , ty_sugg) ; } }
};
}
