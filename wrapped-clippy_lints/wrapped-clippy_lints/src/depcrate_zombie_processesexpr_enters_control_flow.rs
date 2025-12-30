// Generated macro for expr_enters_control_flow (function)
macro_rules! Depcrate_zombie_processesexpr_enters_control_flow {
() => {
// Module: crate::zombie_processes
// Provides: {"expr_enters_control_flow"}
// Dependencies: {}
fn expr_enters_control_flow (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: If (..) | ExprKind :: Match (..) | ExprKind :: Loop (..)) }
};
}
