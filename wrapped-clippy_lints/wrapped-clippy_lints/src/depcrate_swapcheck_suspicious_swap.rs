// Generated macro for check_suspicious_swap (function)
macro_rules! Depcrate_swapcheck_suspicious_swap {
() => {
// Module: crate::swap
// Provides: {"check_suspicious_swap"}
// Dependencies: {}
# [doc = " Implementation of the `ALMOST_SWAPPED` lint."] fn check_suspicious_swap (cx : & LateContext < '_ > , block : & Block < '_ >) { for [first , second] in block . stmts . array_windows () { if let Some ((lhs0 , rhs0)) = parse (first) && let Some ((lhs1 , rhs1)) = parse (second) && first . span . eq_ctxt (second . span) && ! first . span . in_external_macro (cx . sess () . source_map ()) && is_same (cx , lhs0 , rhs1) && is_same (cx , lhs1 , rhs0) && ! is_same (cx , lhs1 , rhs1) && let Some (lhs_sugg) = match & lhs0 { ExprOrIdent :: Expr (expr) => Sugg :: hir_opt (cx , expr) , ExprOrIdent :: Ident (ident) => Some (Sugg :: NonParen (ident . as_str () . into ())) , } && let Some (rhs_sugg) = Sugg :: hir_opt (cx , rhs0) { let span = first . span . to (rhs1 . span) ; let Some (sugg) = std_or_core (cx) else { return } ; span_lint_and_then (cx , ALMOST_SWAPPED , span , format ! ("this looks like you are trying to swap `{lhs_sugg}` and `{rhs_sugg}`") , | diag | { diag . span_suggestion (span , "try" , format ! ("{sugg}::mem::swap({}, {})" , lhs_sugg . mut_addr () , rhs_sugg . mut_addr ()) , Applicability :: MaybeIncorrect ,) ; diag . note (format ! ("or maybe you should use `{sugg}::mem::replace`?")) ; } ,) ; } } }
};
}
