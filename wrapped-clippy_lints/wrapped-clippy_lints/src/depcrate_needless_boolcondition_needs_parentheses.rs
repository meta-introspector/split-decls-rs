// Generated macro for condition_needs_parentheses (function)
macro_rules! Depcrate_needless_boolcondition_needs_parentheses {
() => {
// Module: crate::needless_bool
// Provides: {"condition_needs_parentheses"}
// Dependencies: {}
fn condition_needs_parentheses (e : & Expr < '_ >) -> bool { let mut inner = e ; while let ExprKind :: Binary (_ , i , _) | ExprKind :: Call (i , _) | ExprKind :: Cast (i , _) | ExprKind :: Type (i , _) | ExprKind :: Index (i , _ , _) = inner . kind { if is_block_like (i) { return true ; } inner = i ; } false }
};
}
