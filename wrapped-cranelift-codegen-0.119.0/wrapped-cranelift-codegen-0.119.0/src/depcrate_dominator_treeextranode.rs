// Generated macro for ExtraNode (struct)
macro_rules! Depcrate_dominator_treeExtraNode {
() => {
// Module: crate::dominator_tree
// Provides: {"ExtraNode"}
// Dependencies: {}
# [derive (Default , Clone)] struct ExtraNode { # [doc = " First child node in the domtree."] child : PackedOption < Block > , # [doc = " Next sibling node in the domtree. This linked list is ordered according to the CFG RPO."] sibling : PackedOption < Block > , # [doc = " Sequence number for this node in a pre-order traversal of the dominator tree."] # [doc = " Unreachable blocks have number 0, the entry block is 1."] pre_number : u32 , # [doc = " Maximum `pre_number` for the sub-tree of the dominator tree that is rooted at this node."] # [doc = " This is always >= `pre_number`."] pre_max : u32 , }
};
}
