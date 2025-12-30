// Generated macro for impl_857 (impl)
macro_rules! Depcrate_graph6_graph6_decoderimpl_857 {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"impl_857"}
// Dependencies: {}
# [cfg (feature = "stable_graph")] impl < Ix : IndexType > FromGraph6 for StableGraph < () , () , Undirected , Ix > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : StableGraph < () , () , Undirected , Ix > = StableUnGraph :: with_capacity (order , edges . len ()) ; for _ in 0 .. order { graph . add_node (()) ; } graph . extend_with_edges (edges) ; graph } }
};
}
