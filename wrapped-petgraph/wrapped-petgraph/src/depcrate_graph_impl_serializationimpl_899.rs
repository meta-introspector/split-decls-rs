// Generated macro for impl_899 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_899 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_899"}
// Dependencies: {}
impl < 'de , Ix > Deserialize < 'de > for NodeIndex < Ix > where Ix : IndexType + Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (NodeIndex (Ix :: deserialize (deserializer) ?)) } }
};
}
