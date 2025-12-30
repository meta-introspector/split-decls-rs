// Generated macro for impl_860 (impl)
macro_rules! Depcrate_graph6_graph6_decoderimpl_860 {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"impl_860"}
// Dependencies: {}
impl < Ix : IndexType > FromGraph6 for Csr < () , () , Undirected , Ix > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : Csr < () , () , Undirected , Ix > = Csr :: new () ; let mut nodes = Vec :: new () ; for _ in 0 .. order { let i = graph . add_node (()) ; nodes . push (i) ; } for (a , b) in edges { graph . add_edge (a , b , ()) ; } graph } }
};
}
