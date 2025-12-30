// Generated macro for TraversalEvent (enum)
macro_rules! Depcrate_dominator_treeTraversalEvent {
() => {
// Module: crate::dominator_tree
// Provides: {"TraversalEvent"}
// Dependencies: {}
# [doc = " Traversal event to compute both preorder spanning tree"] # [doc = " and postorder block list. Can't use `Dfs` from traversals.rs"] # [doc = " here because of the need for parent links."] enum TraversalEvent { Enter (u32 , Block) , Exit (Block) , }
};
}
