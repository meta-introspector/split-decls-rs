// Generated macro for check (function)
macro_rules! Depcrate_methods_get_last_with_lencheck {
() => {
// Module: crate::methods::get_last_with_len
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , arg : & Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Sub , .. } , lhs , rhs ,) = arg . kind && let ExprKind :: MethodCall (lhs_path , lhs_recv , [] , _) = & lhs . kind && lhs_path . ident . name == sym :: len && is_integer_literal (rhs , 1) && SpanlessEq :: new (cx) . eq_expr (recv , lhs_recv) && ! recv . can_have_side_effects () { let method = match cx . typeck_results () . expr_ty_adjusted (recv) . peel_refs () . kind () { ty :: Adt (def , _) if cx . tcx . is_diagnostic_item (sym :: VecDeque , def . did ()) => "back" , ty :: Slice (_) => "last" , _ => return , } ; let mut applicability = Applicability :: MachineApplicable ; let recv_snippet = snippet_with_applicability (cx , recv . span , "_" , & mut applicability) ; span_lint_and_sugg (cx , GET_LAST_WITH_LEN , expr . span , format ! ("accessing last element with `{recv_snippet}.get({recv_snippet}.len() - 1)`") , "try" , format ! ("{recv_snippet}.{method}()") , applicability ,) ; } }
};
}
