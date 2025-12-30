// Generated macro for is_never_block (function)
macro_rules! Depcrate_utilsis_never_block {
() => {
// Module: crate::utils
// Provides: {"is_never_block"}
// Dependencies: {}
pub (crate) fn is_never_block (sema : & Semantics < '_ , RootDatabase > , block_expr : & ast :: BlockExpr ,) -> bool { if let Some (tail_expr) = block_expr . tail_expr () { sema . type_of_expr (& tail_expr) . is_some_and (| ty | ty . original . is_never ()) } else if let Some (ast :: Stmt :: ExprStmt (expr_stmt)) = block_expr . statements () . last () && let Some (expr) = expr_stmt . expr () { sema . type_of_expr (& expr) . is_some_and (| ty | ty . original . is_never ()) } else { false } }
};
}
