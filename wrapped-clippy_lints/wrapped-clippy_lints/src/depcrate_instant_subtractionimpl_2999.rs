// Generated macro for impl_2999 (impl)
macro_rules! Depcrate_instant_subtractionimpl_2999 {
() => {
// Module: crate::instant_subtraction
// Provides: {"impl_2999"}
// Dependencies: {}
impl LateLintPass < '_ > for InstantSubtraction { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Sub , .. } , lhs , rhs ,) = expr . kind && let typeck = cx . typeck_results () && ty :: is_type_diagnostic_item (cx , typeck . expr_ty (lhs) , sym :: Instant) { let rhs_ty = typeck . expr_ty (rhs) ; if is_instant_now_call (cx , lhs) && ty :: is_type_diagnostic_item (cx , rhs_ty , sym :: Instant) && let Some (sugg) = Sugg :: hir_opt (cx , rhs) { print_manual_instant_elapsed_sugg (cx , expr , sugg) ; } else if ty :: is_type_diagnostic_item (cx , rhs_ty , sym :: Duration) && ! expr . span . from_expansion () && self . msrv . meets (cx , msrvs :: TRY_FROM) { print_unchecked_duration_subtraction_sugg (cx , lhs , rhs , expr) ; } } } }
};
}
