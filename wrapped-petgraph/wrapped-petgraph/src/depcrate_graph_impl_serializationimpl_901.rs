// Generated macro for impl_901 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_901 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_901"}
// Dependencies: {}
impl < 'de , Ix > Deserialize < 'de > for EdgeIndex < Ix > where Ix : IndexType + Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (EdgeIndex (Ix :: deserialize (deserializer) ?)) } }
};
}
