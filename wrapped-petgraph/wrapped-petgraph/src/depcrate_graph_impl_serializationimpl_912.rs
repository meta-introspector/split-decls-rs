// Generated macro for impl_912 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_912 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_912"}
// Dependencies: {}
# [doc = " Requires crate feature `\"serde-1\"`"] impl < N , E , Ty , Ix > Serialize for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Serialize , N : Serialize , E : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . into_serializable () . serialize (serializer) } }
};
}
