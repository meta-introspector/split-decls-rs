// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_graph_impl_stable_graph_serializationimpl_1094 {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"impl_1094"}
// Dependencies: {}
# [doc = " Requires crate feature `\"serde-1\"`"] impl < N , E , Ty , Ix > Serialize for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Serialize , N : Serialize , E : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . into_serializable () . serialize (serializer) } }
};
}
