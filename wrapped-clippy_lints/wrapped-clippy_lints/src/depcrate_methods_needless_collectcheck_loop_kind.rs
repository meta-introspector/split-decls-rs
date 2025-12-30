// Generated macro for check_loop_kind (function)
macro_rules! Depcrate_methods_needless_collectcheck_loop_kind {
() => {
// Module: crate::methods::needless_collect
// Provides: {"check_loop_kind"}
// Dependencies: {}
fn check_loop_kind < 'tcx > (expr : & Expr < 'tcx >) -> Option < LoopKind < 'tcx > > { if let Some (higher :: WhileLet { let_expr , .. }) = higher :: WhileLet :: hir (expr) { return Some (LoopKind :: Conditional (let_expr)) ; } if let Some (higher :: While { condition , .. }) = higher :: While :: hir (expr) { return Some (LoopKind :: Conditional (condition)) ; } if let Some (higher :: ForLoop { arg , .. }) = higher :: ForLoop :: hir (expr) { return Some (LoopKind :: Conditional (arg)) ; } if let ExprKind :: Loop { .. } = expr . kind { return Some (LoopKind :: Loop) ; } None }
};
}
