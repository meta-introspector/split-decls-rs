// Generated macro for DominatorTree (struct)
macro_rules! Depcrate_dominator_treeDominatorTree {
() => {
// Module: crate::dominator_tree
// Provides: {"DominatorTree"}
// Dependencies: {}
# [doc = " The dominator tree for a single function,"] # [doc = " computed using Semi-NCA algorithm."] pub struct DominatorTree { # [doc = " DFS spanning tree."] stree : SpanningTree , # [doc = " List of CFG blocks in postorder."] postorder : Vec < Block > , # [doc = " Dominator tree nodes."] nodes : SecondaryMap < Block , DominatorTreeNode > , # [doc = " Stack for building the spanning tree."] dfs_worklist : Vec < TraversalEvent > , # [doc = " Stack used for processing semidominator paths"] # [doc = " in link-eval procedure."] eval_worklist : Vec < u32 > , valid : bool , }
};
}
