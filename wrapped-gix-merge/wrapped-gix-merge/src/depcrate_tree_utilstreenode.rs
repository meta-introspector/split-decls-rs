// Generated macro for TreeNode (struct)
macro_rules! Depcrate_tree_utilsTreeNode {
() => {
// Module: crate::tree::utils
// Provides: {"TreeNode"}
// Dependencies: {}
# [doc = " Trees lead to other trees, or leafs (without children), and it can be represented by a renamed directory."] # [derive (Debug , Default , Clone)] struct TreeNode { # [doc = " A mapping of path components to their children to quickly see if `theirs` in some way is potentially"] # [doc = " conflicting with `ours`."] children : HashMap < BString , usize > , # [doc = " The index to a change, which is always set if this is a leaf node (with no children), and if there are children and this"] # [doc = " is a rewritten tree."] change_idx : Option < usize > , # [doc = " Keep track of where the location of this node is derived from."] location : ChangeLocation , }
};
}
