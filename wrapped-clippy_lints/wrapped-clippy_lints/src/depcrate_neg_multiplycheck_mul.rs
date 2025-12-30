// Generated macro for check_mul (function)
macro_rules! Depcrate_neg_multiplycheck_mul {
() => {
// Module: crate::neg_multiply
// Provides: {"check_mul"}
// Dependencies: {}
fn check_mul (cx : & LateContext < '_ > , mul_expr : & Expr < '_ > , lit : & Expr < '_ > , exp : & Expr < '_ >) { const F16_ONE : u16 = 1.0_f16 . to_bits () ; const F128_ONE : u128 = 1.0_f128 . to_bits () ; if let ExprKind :: Lit (l) = lit . kind && matches ! (consts :: lit_to_mir_constant (& l . node , cx . typeck_results () . expr_ty_opt (lit)) , Constant :: Int (1) | Constant :: F16 (F16_ONE) | Constant :: F32 (1.0) | Constant :: F64 (1.0) | Constant :: F128 (F128_ONE)) && cx . typeck_results () . expr_ty (exp) . is_numeric () { let mut applicability = Applicability :: MachineApplicable ; let (snip , from_macro) = snippet_with_context (cx , exp . span , mul_expr . span . ctxt () , ".." , & mut applicability) ; let needs_parens_for_postfix = is_in_parens_with_postfix (cx , mul_expr) ; let suggestion = if needs_parens_for_postfix { format ! ("(-{snip})") } else if ! from_macro && cx . precedence (exp) < ExprPrecedence :: Prefix && ! has_enclosing_paren (& snip) { format ! ("-({snip})") } else { format ! ("-{snip}") } ; span_lint_and_sugg (cx , NEG_MULTIPLY , mul_expr . span , "this multiplication by -1 can be written more succinctly" , "consider using" , suggestion , applicability ,) ; } }
};
}
