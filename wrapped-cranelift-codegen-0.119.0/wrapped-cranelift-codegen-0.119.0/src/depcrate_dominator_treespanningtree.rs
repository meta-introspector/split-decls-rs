// Generated macro for SpanningTree (struct)
macro_rules! Depcrate_dominator_treeSpanningTree {
() => {
// Module: crate::dominator_tree
// Provides: {"SpanningTree"}
// Dependencies: {}
# [doc = " Spanning tree, in CFG preorder."] # [doc = " Node 0 is the virtual root and doesn't have a corresponding block."] # [doc = " It's not required because function's CFG in Cranelift always have"] # [doc = " a singular root, but helps to avoid additional checks."] # [doc = " Numbering nodes from 0 also follows the convention in"] # [doc = " `SimpleDominatorTree` and `DominatorTreePreorder`."] # [derive (Clone , Default)] struct SpanningTree { nodes : Vec < SpanningTreeNode > , }
};
}
