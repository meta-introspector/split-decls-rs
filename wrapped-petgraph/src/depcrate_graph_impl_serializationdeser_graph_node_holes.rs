// Generated macro for deser_graph_node_holes (function)
macro_rules! Depcrate_graph_impl_serializationdeser_graph_node_holes {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"deser_graph_node_holes"}
// Dependencies: {}
fn deser_graph_node_holes < 'de , D , Ix > (deserializer : D) -> Result < Vec < NodeIndex < Ix > > , D :: Error > where D : Deserializer < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: < NodeIndex < Ix > , NodeIndex < Ix > , _ > :: new (| _ | { Err ("Graph can not have holes in the node set, found non-empty node_holes") }) ,) }
};
}
