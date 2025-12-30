// Generated macro for impl_3025 (impl)
macro_rules! Depcrate_integer_division_remainder_usedimpl_3025 {
() => {
// Module: crate::integer_division_remainder_used
// Provides: {"impl_3025"}
// Dependencies: {}
impl LateLintPass < '_ > for IntegerDivisionRemainderUsed { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Binary (op , lhs , rhs) = & expr . kind && let BinOpKind :: Div | BinOpKind :: Rem = op . node && let lhs_ty = cx . typeck_results () . expr_ty (lhs) && let rhs_ty = cx . typeck_results () . expr_ty (rhs) && let ty :: Int (_) | ty :: Uint (_) = lhs_ty . peel_refs () . kind () && let ty :: Int (_) | ty :: Uint (_) = rhs_ty . peel_refs () . kind () { span_lint (cx , INTEGER_DIVISION_REMAINDER_USED , expr . span . source_callsite () , format ! ("use of {} has been disallowed in this context" , op . node . as_str ()) ,) ; } } }
};
}
