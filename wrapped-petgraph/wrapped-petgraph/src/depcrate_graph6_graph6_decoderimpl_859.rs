// Generated macro for impl_859 (impl)
macro_rules! Depcrate_graph6_graph6_decoderimpl_859 {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"impl_859"}
// Dependencies: {}
# [cfg (feature = "matrix_graph")] impl < Null , Ix , S > FromGraph6 for MatrixGraph < () , () , S , Undirected , Null , Ix > where Null : Nullable < Wrapped = () > , Ix : IndexType , S : BuildHasher + Default , { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : MatrixGraph < () , () , S , Undirected , Null , Ix > = MatrixGraph :: with_capacity (order) ; for _ in 0 .. order { graph . add_node (()) ; } graph . extend_with_edges (edges . iter ()) ; graph } }
};
}
