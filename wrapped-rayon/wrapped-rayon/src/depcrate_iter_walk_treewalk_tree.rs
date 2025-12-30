// Generated macro for walk_tree (function)
macro_rules! Depcrate_iter_walk_treewalk_tree {
() => {
// Module: crate::iter::walk_tree
// Provides: {"walk_tree"}
// Dependencies: {}
# [doc = " Create a tree like parallel iterator from an initial root node."] # [doc = " The `children_of` function should take a node and iterate on all of its child nodes."] # [doc = " The best parallelization is obtained when the tree is balanced"] # [doc = " but we should also be able to handle harder cases."] # [doc = ""] # [doc = " # Ordering"] # [doc = ""] # [doc = " This function does not guarantee any ordering but will"] # [doc = " use whatever algorithm is thought to achieve the fastest traversal."] # [doc = " See also [`walk_tree_prefix`] which guarantees a"] # [doc = " prefix order and [`walk_tree_postfix`] which guarantees a postfix order."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```text"] # [doc = "      4"] # [doc = "     / \\"] # [doc = "    /   \\"] # [doc = "   2     3"] # [doc = "        / \\"] # [doc = "       1   2"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::iter::walk_tree;"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let par_iter = walk_tree(4, |&e| {"] # [doc = "     if e <= 2 {"] # [doc = "         Vec::new()"] # [doc = "     } else {"] # [doc = "         vec![e / 2, e / 2 + 1]"] # [doc = "     }"] # [doc = " });"] # [doc = " assert_eq!(par_iter.sum::<u32>(), 12);"] # [doc = " ```"] pub fn walk_tree < S , B , I > (root : S , children_of : B) -> WalkTree < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S , IntoIter : DoubleEndedIterator > , { let walker = WalkTreePostfix { initial_state : root , children_of , } ; WalkTree (walker) }
};
}
