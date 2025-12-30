// Generated macro for block_parents_have_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksblock_parents_have_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"block_parents_have_safety_comment"}
// Dependencies: {}
fn block_parents_have_safety_comment (accept_comment_above_statement : bool , accept_comment_above_attributes : bool , cx : & LateContext < '_ > , id : HirId ,) -> bool { let span = match cx . tcx . parent_hir_node (id) { Node :: Expr (expr) if let Some ((span , _)) = find_unsafe_block_parent_in_expr (cx , expr) => span , Node :: Stmt (hir :: Stmt { kind : hir :: StmtKind :: Let (hir :: LetStmt { span , .. }) | hir :: StmtKind :: Expr (hir :: Expr { span , .. }) | hir :: StmtKind :: Semi (hir :: Expr { span , .. }) , .. }) | Node :: LetStmt (hir :: LetStmt { span , .. }) => * span , node if let Some ((span , _)) = span_and_hid_of_item_alike_node (& node) && is_const_or_static (& node) => { span } , _ => return false , } ; accept_comment_above_statement && span_has_safety_comment (cx , span , accept_comment_above_attributes) }
};
}
