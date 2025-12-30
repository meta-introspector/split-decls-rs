// Generated macro for subgraph_edges_from_metric_closure (function)
macro_rules! Depcrate_algo_steiner_treesubgraph_edges_from_metric_closure {
() => {
// Module: crate::algo::steiner_tree
// Provides: {"subgraph_edges_from_metric_closure"}
// Dependencies: {}
fn subgraph_edges_from_metric_closure < G > (graph : G , minimum_spanning_closure : G ,) -> (Vec < Edge < G > > , Subgraph < G >) where G : GraphBase + NodeCompactIndexable + IntoEdgeReferences + IntoNodeIdentifiers + GraphProp + IntoNodeReferences , G :: EdgeWeight : BoundedMeasure + Copy , G :: NodeId : Eq + Hash + Ord + Debug , { let mut retained_nodes = HashSet :: new () ; let mut retained_edges = Vec :: new () ; let (_ , prev) = floyd_warshall_path (graph , | e | * e . weight ()) . unwrap () ; for edge in minimum_spanning_closure . edge_references () { let target = graph . to_index (edge . target ()) ; let source = graph . to_index (edge . source ()) ; let mut current = target ; while current != source { if let Some (prev_node) = prev [source] [current] { retained_nodes . insert (graph . from_index (prev_node)) ; retained_nodes . insert (graph . from_index (current)) ; retained_edges . push ((graph . from_index (prev_node) , graph . from_index (current))) ; current = prev_node ; } } } (retained_edges , retained_nodes) }
};
}
