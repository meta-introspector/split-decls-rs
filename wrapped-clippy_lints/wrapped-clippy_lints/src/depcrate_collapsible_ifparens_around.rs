// Generated macro for parens_around (function)
macro_rules! Depcrate_collapsible_ifparens_around {
() => {
// Module: crate::collapsible_if
// Provides: {"parens_around"}
// Dependencies: {}
# [doc = " If the expression is a `||`, suggest parentheses around it."] fn parens_around (expr : & Expr < '_ >) -> Vec < (Span , String) > { if let ExprKind :: Binary (op , _ , _) = expr . peel_drop_temps () . kind && op . node == BinOpKind :: Or { vec ! [(expr . span . shrink_to_lo () , String :: from ("(")) , (expr . span . shrink_to_hi () , String :: from (")")) ,] } else { vec ! [] } }
};
}
