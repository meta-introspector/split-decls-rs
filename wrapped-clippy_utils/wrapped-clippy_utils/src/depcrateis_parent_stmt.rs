// Generated macro for is_parent_stmt (function)
macro_rules! Depcrateis_parent_stmt {
() => {
// Module: crate
// Provides: {"is_parent_stmt"}
// Dependencies: {}
# [doc = " Returns true if the specified `HirId` is the top-level expression of a statement or the only"] # [doc = " expression in a block."] pub fn is_parent_stmt (cx : & LateContext < '_ > , id : HirId) -> bool { matches ! (cx . tcx . parent_hir_node (id) , Node :: Stmt (..) | Node :: Block (Block { stmts : [] , .. })) }
};
}
