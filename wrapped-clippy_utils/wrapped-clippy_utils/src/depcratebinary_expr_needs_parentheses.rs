// Generated macro for binary_expr_needs_parentheses (function)
macro_rules! Depcratebinary_expr_needs_parentheses {
() => {
// Module: crate
// Provides: {"binary_expr_needs_parentheses"}
// Dependencies: {}
# [doc = " Returns true if the given `expr` is binary expression that needs to be wrapped in parentheses."] pub fn binary_expr_needs_parentheses (expr : & Expr < '_ >) -> bool { fn contains_block (expr : & Expr < '_ > , is_operand : bool) -> bool { match expr . kind { ExprKind :: Binary (_ , lhs , _) | ExprKind :: Cast (lhs , _) => contains_block (lhs , true) , _ if is_block_like (expr) => is_operand , _ => false , } } contains_block (expr , false) }
};
}
