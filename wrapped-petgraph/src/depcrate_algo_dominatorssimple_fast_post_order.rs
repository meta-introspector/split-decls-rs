// Generated macro for simple_fast_post_order (function)
macro_rules! Depcrate_algo_dominatorssimple_fast_post_order {
() => {
// Module: crate::algo::dominators
// Provides: {"simple_fast_post_order"}
// Dependencies: {}
fn simple_fast_post_order < G > (graph : G , root : G :: NodeId ,) -> (Vec < G :: NodeId > , PredecessorSets < G :: NodeId >) where G : IntoNeighbors + Visitable , < G as GraphBase > :: NodeId : Eq + Hash , { let mut post_order = vec ! [] ; let mut predecessor_sets = HashMap :: new () ; for node in DfsPostOrder :: new (graph , root) . iter (graph) { post_order . push (node) ; for successor in graph . neighbors (node) { predecessor_sets . entry (successor) . or_insert_with (HashSet :: new) . insert (node) ; } } (post_order , predecessor_sets) }
};
}
