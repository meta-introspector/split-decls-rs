// Generated macro for fetch_bool_block (function)
macro_rules! Depcrate_needless_boolfetch_bool_block {
() => {
// Module: crate::needless_bool
// Provides: {"fetch_bool_block"}
// Dependencies: {}
fn fetch_bool_block (expr : & Expr < '_ >) -> Option < Expression > { match peel_blocks_with_stmt (expr) . kind { ExprKind :: Ret (Some (ret)) => Some (Expression :: RetBool (fetch_bool_expr (ret) ?)) , _ => Some (Expression :: Bool (fetch_bool_expr (expr) ?)) , } }
};
}
