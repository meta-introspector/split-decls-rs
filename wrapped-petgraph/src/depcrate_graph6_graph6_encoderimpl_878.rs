// Generated macro for impl_878 (impl)
macro_rules! Depcrate_graph6_graph6_encoderimpl_878 {
() => {
// Module: crate::graph6::graph6_encoder
// Provides: {"impl_878"}
// Dependencies: {}
# [cfg (feature = "matrix_graph")] impl < N , E , S , Null , Ix > ToGraph6 for MatrixGraph < N , E , S , Undirected , Null , Ix > where N : NodeTrait , Null : Nullable < Wrapped = E > , Ix : IndexType , S : BuildHasher + Default , { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
};
}
