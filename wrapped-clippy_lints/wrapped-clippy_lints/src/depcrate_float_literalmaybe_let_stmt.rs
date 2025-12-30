// Generated macro for maybe_let_stmt (function)
macro_rules! Depcrate_float_literalmaybe_let_stmt {
() => {
// Module: crate::float_literal
// Provides: {"maybe_let_stmt"}
// Dependencies: {}
fn maybe_let_stmt < 'a > (cx : & LateContext < 'a > , expr : & hir :: Expr < '_ >) -> Option < & 'a hir :: LetStmt < 'a > > { let parent = cx . tcx . parent_hir_node (expr . hir_id) ; match parent { hir :: Node :: LetStmt (let_stmt) => Some (let_stmt) , _ => None , } }
};
}
