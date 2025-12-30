// Generated macro for Tree (struct)
macro_rules! Depcrate_treeTree {
() => {
// Module: crate::tree
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " A tree abstraction, intended for fast building as a preorder traversal."] # [derive (Clone)] pub (crate) struct Tree < T > { nodes : Vec < Node < T > > , spine : Vec < TreeIndex > , cur : Option < TreeIndex > , }
};
}
