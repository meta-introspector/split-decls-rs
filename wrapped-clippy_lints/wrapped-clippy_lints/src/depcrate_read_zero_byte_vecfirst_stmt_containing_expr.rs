// Generated macro for first_stmt_containing_expr (function)
macro_rules! Depcrate_read_zero_byte_vecfirst_stmt_containing_expr {
() => {
// Module: crate::read_zero_byte_vec
// Provides: {"first_stmt_containing_expr"}
// Dependencies: {}
fn first_stmt_containing_expr < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx hir :: Stmt < 'tcx > > { cx . tcx . hir_parent_iter (expr . hir_id) . find_map (| (_ , node) | { if let hir :: Node :: Stmt (stmt) = node { Some (stmt) } else { None } }) }
};
}
