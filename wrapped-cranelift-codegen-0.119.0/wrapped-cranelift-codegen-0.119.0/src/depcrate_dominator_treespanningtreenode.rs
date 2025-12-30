// Generated macro for SpanningTreeNode (struct)
macro_rules! Depcrate_dominator_treeSpanningTreeNode {
() => {
// Module: crate::dominator_tree
// Provides: {"SpanningTreeNode"}
// Dependencies: {}
# [doc = " Spanning tree node, used during domtree computation."] # [derive (Clone , Default)] struct SpanningTreeNode { # [doc = " This node's block in function CFG."] block : PackedOption < Block > , # [doc = " Node's ancestor in the spanning tree."] # [doc = " Gets invalidated during semi-dominator computation."] ancestor : u32 , # [doc = " The smallest semi value discovered on any semi-dominator path"] # [doc = " that went through the node up till the moment."] # [doc = " Gets updated in the course of semi-dominator computation."] label : u32 , # [doc = " Semidominator value for the node."] semi : u32 , # [doc = " Immediate dominator value for the node."] # [doc = " Initialized to node's ancestor in the spanning tree."] idom : u32 , }
};
}
