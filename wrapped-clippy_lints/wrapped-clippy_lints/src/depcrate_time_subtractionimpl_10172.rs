// Generated macro for impl_10172 (impl)
macro_rules! Depcrate_time_subtractionimpl_10172 {
() => {
// Module: crate::time_subtraction
// Provides: {"impl_10172"}
// Dependencies: {}
impl LateLintPass < '_ > for UncheckedTimeSubtraction { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Sub , .. } , lhs , rhs ,) = expr . kind { let typeck = cx . typeck_results () ; let lhs_ty = typeck . expr_ty (lhs) ; let rhs_ty = typeck . expr_ty (rhs) ; if lhs_ty . is_diag_item (cx , sym :: Instant) { if is_instant_now_call (cx , lhs) && rhs_ty . is_diag_item (cx , sym :: Instant) && let Some (sugg) = Sugg :: hir_opt (cx , rhs) { print_manual_instant_elapsed_sugg (cx , expr , sugg) ; } else if rhs_ty . is_diag_item (cx , sym :: Duration) && ! expr . span . from_expansion () && self . msrv . meets (cx , msrvs :: TRY_FROM) { print_unchecked_duration_subtraction_sugg (cx , lhs , rhs , expr) ; } } else if lhs_ty . is_diag_item (cx , sym :: Duration) && rhs_ty . is_diag_item (cx , sym :: Duration) && ! expr . span . from_expansion () && self . msrv . meets (cx , msrvs :: TRY_FROM) { print_unchecked_duration_subtraction_sugg (cx , lhs , rhs , expr) ; } } } }
};
}
