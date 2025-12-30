// Generated macro for compute_shortest_path_length (function)
macro_rules! Depcrate_algo_steiner_treecompute_shortest_path_length {
() => {
// Module: crate::algo::steiner_tree
// Provides: {"compute_shortest_path_length"}
// Dependencies: {}
fn compute_shortest_path_length < G > (graph : G , source : G :: NodeId , target : G :: NodeId) -> G :: EdgeWeight where G : Visitable + IntoEdges , G :: NodeId : Eq + Hash , G :: EdgeWeight : Measure + Copy , { let output = dijkstra (graph , source , Some (target) , | e | * e . weight ()) ; output [& target] }
};
}
