// Generated macro for is_in_parens_with_postfix (function)
macro_rules! Depcrate_neg_multiplyis_in_parens_with_postfix {
() => {
// Module: crate::neg_multiply
// Provides: {"is_in_parens_with_postfix"}
// Dependencies: {}
fn is_in_parens_with_postfix (cx : & LateContext < '_ > , mul_expr : & Expr < '_ >) -> bool { if let Some (parent) = get_parent_expr (cx , mul_expr) { let mult_snippet = snippet (cx , mul_expr . span , "") ; if has_enclosing_paren (& mult_snippet) && let ExprKind :: MethodCall (_ , _ , _ , _) = parent . kind { return true ; } } false }
};
}
