// Generated macro for check_literal (function)
macro_rules! Depcrate_operators_manual_div_ceilcheck_literal {
() => {
// Module: crate::operators::manual_div_ceil
// Provides: {"check_literal"}
// Dependencies: {}
fn check_literal (expr : & Expr < '_ >) -> bool { if let ExprKind :: Lit (lit) = expr . kind && let LitKind :: Int (Pu128 (1) , _) = lit . node { return true ; } false }
};
}
