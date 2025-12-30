// Generated macro for impl_10939 (impl)
macro_rules! Depcrate_unnecessary_semicolonimpl_10939 {
() => {
// Module: crate::unnecessary_semicolon
// Provides: {"impl_10939"}
// Dependencies: {}
impl UnnecessarySemicolon { # [doc = " Enter or leave a block, remembering the last statement of the block."] fn handle_block (& mut self , cx : & LateContext < '_ > , block : & Block < '_ > , enter : bool) { if block . expr . is_none () && let Some (last_stmt) = block . stmts . last () { if enter { let block_ty = cx . typeck_results () . node_type (block . hir_id) ; self . last_statements . push ((last_stmt . hir_id , block_ty . is_unit ())) ; } else { self . last_statements . pop () ; } } } # [doc = " Checks if `stmt` is the last statement in an expressionless block. In this case,"] # [doc = " return `Some` with a boolean which is `true` if the block type is `()`."] fn is_last_in_block (& self , stmt : & Stmt < '_ >) -> Option < bool > { self . last_statements . last () . and_then (| & (stmt_id , is_unit) | (stmt_id == stmt . hir_id) . then_some (is_unit)) } }
};
}
