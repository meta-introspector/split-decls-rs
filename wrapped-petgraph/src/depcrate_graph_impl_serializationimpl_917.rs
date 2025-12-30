// Generated macro for impl_917 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_917 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_917"}
// Dependencies: {}
# [doc = " Requires crate feature `\"serde-1\"`"] impl < 'de , N , E , Ty , Ix > Deserialize < 'de > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Deserialize < 'de > , N : Deserialize < 'de > , E : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Self :: from_deserialized (DeserGraph :: deserialize (deserializer) ?) } }
};
}
