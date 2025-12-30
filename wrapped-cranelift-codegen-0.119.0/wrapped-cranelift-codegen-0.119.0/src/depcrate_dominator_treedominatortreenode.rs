// Generated macro for DominatorTreeNode (struct)
macro_rules! Depcrate_dominator_treeDominatorTreeNode {
() => {
// Module: crate::dominator_tree
// Provides: {"DominatorTreeNode"}
// Dependencies: {}
# [doc = " Dominator tree node. We keep one of these per block."] # [derive (Clone , Default)] struct DominatorTreeNode { # [doc = " Immediate dominator for the block, `None` for unreachable blocks."] idom : PackedOption < Block > , # [doc = " Preorder traversal number, zero for unreachable blocks."] pre_number : u32 , }
};
}
