// Generated macro for predecessor_sets_to_idx_vecs (function)
macro_rules! Depcrate_algo_dominatorspredecessor_sets_to_idx_vecs {
() => {
// Module: crate::algo::dominators
// Provides: {"predecessor_sets_to_idx_vecs"}
// Dependencies: {}
fn predecessor_sets_to_idx_vecs < N > (post_order : & [N] , node_to_post_order_idx : & HashMap < N , usize > , mut predecessor_sets : HashMap < N , HashSet < N > > ,) -> Vec < Vec < usize > > where N : Copy + Eq + Hash , { post_order . iter () . map (| node | { predecessor_sets . remove (node) . map (| predecessors | { predecessors . into_iter () . map (| p | * node_to_post_order_idx . get (& p) . unwrap ()) . collect () }) . unwrap_or_default () }) . collect () }
};
}
