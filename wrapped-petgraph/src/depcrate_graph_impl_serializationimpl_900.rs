// Generated macro for impl_900 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_900 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_900"}
// Dependencies: {}
impl < Ix > Serialize for EdgeIndex < Ix > where Ix : IndexType + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
