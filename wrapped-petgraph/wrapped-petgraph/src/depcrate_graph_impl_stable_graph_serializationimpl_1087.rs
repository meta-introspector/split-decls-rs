// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_graph_impl_stable_graph_serializationimpl_1087 {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"impl_1087"}
// Dependencies: {}
impl < N , Ix > Serialize for Somes < & [Node < Option < N > , Ix >] > where N : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq_with_length (self . 0 , self . 1 . iter () . filter_map (| node | node . weight . as_ref ()) ,) } }
};
}
