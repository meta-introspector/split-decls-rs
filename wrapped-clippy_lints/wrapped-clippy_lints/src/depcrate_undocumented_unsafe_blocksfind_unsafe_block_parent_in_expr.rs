// Generated macro for find_unsafe_block_parent_in_expr (function)
macro_rules! Depcrate_undocumented_unsafe_blocksfind_unsafe_block_parent_in_expr {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"find_unsafe_block_parent_in_expr"}
// Dependencies: {}
fn find_unsafe_block_parent_in_expr < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > ,) -> Option < (Span , HirId) > { match cx . tcx . parent_hir_node (expr . hir_id) { Node :: LetStmt (hir :: LetStmt { span , hir_id , .. }) | Node :: Expr (hir :: Expr { hir_id , kind : hir :: ExprKind :: Assign (_ , _ , span) , .. }) => Some ((* span , * hir_id)) , Node :: Expr (expr) => find_unsafe_block_parent_in_expr (cx , expr) , node if let Some ((span , hir_id)) = span_and_hid_of_item_alike_node (& node) && is_const_or_static (& node) => { Some ((span , hir_id)) } , _ => { if is_branchy (expr) { return None ; } Some ((expr . span , expr . hir_id)) } , } }
};
}
