// Generated macro for check (function)
macro_rules! Depcrate_operators_integer_division_remainder_usedcheck {
() => {
// Module: crate::operators::integer_division_remainder_used
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , op : BinOpKind , lhs : & Expr < '_ > , rhs : & Expr < '_ > , span : Span) { if let BinOpKind :: Div | BinOpKind :: Rem = op && let lhs_ty = cx . typeck_results () . expr_ty (lhs) && let rhs_ty = cx . typeck_results () . expr_ty (rhs) && let ty :: Int (_) | ty :: Uint (_) = lhs_ty . peel_refs () . kind () && let ty :: Int (_) | ty :: Uint (_) = rhs_ty . peel_refs () . kind () { span_lint (cx , INTEGER_DIVISION_REMAINDER_USED , span . source_callsite () , format ! ("use of `{}` has been disallowed in this context" , op . as_str ()) ,) ; } }
};
}
