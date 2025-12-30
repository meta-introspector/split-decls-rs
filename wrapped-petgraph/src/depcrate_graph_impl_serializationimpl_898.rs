// Generated macro for impl_898 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_898 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_898"}
// Dependencies: {}
impl < Ix > Serialize for NodeIndex < Ix > where Ix : IndexType + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
