// Generated macro for impl_856 (impl)
macro_rules! Depcrate_graph6_graph6_decoderimpl_856 {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"impl_856"}
// Dependencies: {}
impl < Ix : IndexType > FromGraph6 for Graph < () , () , Undirected , Ix > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : Graph < () , () , Undirected , Ix > = Graph :: with_capacity (order , edges . len ()) ; for _ in 0 .. order { graph . add_node (()) ; } graph . extend_with_edges (edges) ; graph } }
};
}
