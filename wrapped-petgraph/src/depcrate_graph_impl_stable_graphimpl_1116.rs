// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1116 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1116"}
// Dependencies: {}
# [doc = " Convert a `StableGraph` into a `Graph`"] # [doc = ""] # [doc = " Computes in **O(|V| + |E|)** time where V is the set of nodes and E is the set of edges."] # [doc = ""] # [doc = " This translates the stable graph into a graph with node and edge indices in"] # [doc = " a compact interval without holes (like `Graph`s always are)."] # [doc = ""] # [doc = " Only if the stable graph had no vacancies after deletions (if node bound was"] # [doc = " equal to node count, and the same for edges), would the resulting graph have"] # [doc = " the same node and edge indices as the input."] impl < N , E , Ty , Ix > From < StableGraph < N , E , Ty , Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn from (graph : StableGraph < N , E , Ty , Ix >) -> Self { let mut result_g = Graph :: with_capacity (graph . node_count () , graph . edge_count ()) ; let mut node_index_map = vec ! [NodeIndex :: end () ; graph . node_bound ()] ; for (i , node) in graph . g . nodes . into_iter () . enumerate () { if let Some (nw) = node . weight { node_index_map [i] = result_g . add_node (nw) ; } } for edge in graph . g . edges { let source_index = edge . source () . index () ; let target_index = edge . target () . index () ; if let Some (ew) = edge . weight { let source = node_index_map [source_index] ; let target = node_index_map [target_index] ; debug_assert ! (source != NodeIndex :: end ()) ; debug_assert ! (target != NodeIndex :: end ()) ; result_g . add_edge (source , target , ew) ; } } result_g } }
};
}
