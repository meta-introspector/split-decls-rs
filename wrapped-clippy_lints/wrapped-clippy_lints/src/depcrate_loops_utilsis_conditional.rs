// Generated macro for is_conditional (function)
macro_rules! Depcrate_loops_utilsis_conditional {
() => {
// Module: crate::loops::utils
// Provides: {"is_conditional"}
// Dependencies: {}
fn is_conditional (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: If (..) | ExprKind :: Match (..)) }
};
}
