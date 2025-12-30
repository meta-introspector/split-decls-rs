// Generated macro for peel_hir_expr_unary (function)
macro_rules! Depcratepeel_hir_expr_unary {
() => {
// Module: crate
// Provides: {"peel_hir_expr_unary"}
// Dependencies: {}
# [doc = " Peels off all unary operators of an expression. Returns the underlying expression and the number"] # [doc = " of operators removed."] pub fn peel_hir_expr_unary < 'a > (expr : & 'a Expr < 'a >) -> (& 'a Expr < 'a > , usize) { let mut count : usize = 0 ; let mut curr_expr = expr ; while let ExprKind :: Unary (_ , local_expr) = curr_expr . kind { count = count . wrapping_add (1) ; curr_expr = local_expr ; } (curr_expr , count) }
};
}
