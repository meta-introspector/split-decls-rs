// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_graph_impl_stable_graph_serializationimpl_1096 {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"impl_1096"}
// Dependencies: {}
# [doc = " Requires crate feature `\"serde-1\"`"] impl < 'de , N , E , Ty , Ix > Deserialize < 'de > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Deserialize < 'de > , N : Deserialize < 'de > , E : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Self :: from_deserialized (DeserStableGraph :: deserialize (deserializer) ?) } }
};
}
