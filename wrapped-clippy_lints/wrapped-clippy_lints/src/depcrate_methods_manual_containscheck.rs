// Generated macro for check (function)
macro_rules! Depcrate_methods_manual_containscheck {
() => {
// Module: crate::methods::manual_contains
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , closure_arg : & Expr < '_ >) { let mut app = Applicability :: MachineApplicable ; if ! expr . span . from_expansion () && let ExprKind :: Closure (closure) = closure_arg . kind && let Body { params : [param] , value } = cx . tcx . hir_body (closure . body) && let ExprKind :: Binary (op , lhs , rhs) = value . kind && let (peeled_ref_pat , _) = peel_hir_pat_refs (param . pat) && let Some ((snip , snip_expr)) = can_replace_with_contains (cx , op , lhs , rhs , peeled_ref_pat . hir_id , & mut app) && let ref_type = cx . typeck_results () . expr_ty_adjusted (recv) && let ty :: Ref (_ , inner_type , _) = ref_type . kind () && let ty :: Slice (slice_type) = inner_type . kind () && * slice_type == cx . typeck_results () . expr_ty (snip_expr) { span_lint_and_sugg (cx , MANUAL_CONTAINS , expr . span , "using `contains()` instead of `iter().any()` is more efficient" , "try" , format ! ("{}.contains({})" , snippet_with_applicability (cx , recv . span , "_" , & mut app) , snip) , app ,) ; } }
};
}
