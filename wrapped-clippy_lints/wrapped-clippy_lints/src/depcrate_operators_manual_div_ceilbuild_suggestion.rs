// Generated macro for build_suggestion (function)
macro_rules! Depcrate_operators_manual_div_ceilbuild_suggestion {
() => {
// Module: crate::operators::manual_div_ceil
// Provides: {"build_suggestion"}
// Dependencies: {}
fn build_suggestion (cx : & LateContext < '_ > , expr : & Expr < '_ > , lhs : & Expr < '_ > , rhs : & Expr < '_ > , applicability : & mut Applicability ,) { let dividend_sugg = Sugg :: hir_with_applicability (cx , lhs , ".." , applicability) . maybe_paren () ; let rhs_ty = cx . typeck_results () . expr_ty (rhs) ; let type_suffix = if cx . typeck_results () . expr_ty (lhs) . is_numeric () && matches ! (lhs . kind , ExprKind :: Lit (Spanned { node : LitKind :: Int (_ , LitIntType :: Unsuffixed) , .. }) | ExprKind :: Unary (UnOp :: Neg , Expr { kind : ExprKind :: Lit (Spanned { node : LitKind :: Int (_ , LitIntType :: Unsuffixed) , .. }) , .. })) { format ! ("_{rhs_ty}") } else { String :: new () } ; let dividend_sugg_str = dividend_sugg . into_string () ; let suggestion_before_div_ceil = if has_enclosing_paren (& dividend_sugg_str) { format ! ("{}{})" , & dividend_sugg_str [.. dividend_sugg_str . len () - 1] . to_string () , type_suffix) } else { format ! ("{dividend_sugg_str}{type_suffix}") } ; let divisor_snippet = match Sugg :: hir_with_context (cx , rhs , expr . span . ctxt () , "_" , applicability) { sugg if rhs_ty . is_ref () => sugg . deref () , sugg => sugg , } ; span_lint_and_sugg (cx , MANUAL_DIV_CEIL , expr . span , "manually reimplementing `div_ceil`" , "consider using `.div_ceil()`" , format ! ("{suggestion_before_div_ceil}.div_ceil({divisor_snippet})") , * applicability ,) ; }
};
}
