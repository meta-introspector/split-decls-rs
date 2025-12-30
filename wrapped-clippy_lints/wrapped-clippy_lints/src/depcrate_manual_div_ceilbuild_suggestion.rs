// Generated macro for build_suggestion (function)
macro_rules! Depcrate_manual_div_ceilbuild_suggestion {
() => {
// Module: crate::manual_div_ceil
// Provides: {"build_suggestion"}
// Dependencies: {}
fn build_suggestion (cx : & LateContext < '_ > , expr : & Expr < '_ > , lhs : & Expr < '_ > , rhs : & Expr < '_ > , applicability : & mut Applicability ,) { let dividend_sugg = Sugg :: hir_with_applicability (cx , lhs , ".." , applicability) . maybe_paren () ; let type_suffix = if cx . typeck_results () . expr_ty (lhs) . is_numeric () && matches ! (lhs . kind , ExprKind :: Lit (Spanned { node : LitKind :: Int (_ , LitIntType :: Unsuffixed) , .. }) | ExprKind :: Unary (UnOp :: Neg , Expr { kind : ExprKind :: Lit (Spanned { node : LitKind :: Int (_ , LitIntType :: Unsuffixed) , .. }) , .. })) { format ! ("_{}" , cx . typeck_results () . expr_ty (rhs)) } else { String :: new () } ; let dividend_sugg_str = dividend_sugg . into_string () ; let suggestion_before_div_ceil = if has_enclosing_paren (& dividend_sugg_str) { format ! ("{}{})" , & dividend_sugg_str [.. dividend_sugg_str . len () - 1] . to_string () , type_suffix) } else { format ! ("{dividend_sugg_str}{type_suffix}") } ; let divisor_snippet = snippet_with_context (cx , rhs . span , expr . span . ctxt () , ".." , applicability) ; let sugg = format ! ("{suggestion_before_div_ceil}.div_ceil({})" , divisor_snippet . 0) ; span_lint_and_sugg (cx , MANUAL_DIV_CEIL , expr . span , "manually reimplementing `div_ceil`" , "consider using `.div_ceil()`" , sugg , * applicability ,) ; }
};
}
