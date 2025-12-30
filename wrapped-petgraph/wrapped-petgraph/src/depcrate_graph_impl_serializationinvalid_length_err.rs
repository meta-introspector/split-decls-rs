// Generated macro for invalid_length_err (function)
macro_rules! Depcrate_graph_impl_serializationinvalid_length_err {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"invalid_length_err"}
// Dependencies: {}
pub fn invalid_length_err < Ix , E > (node_or_edge : & str , len : usize) -> E where E : Error , Ix : IndexType , { E :: custom (format_args ! ("invalid size: graph {} count {} exceeds index type maximum {}" , node_or_edge , len , < Ix as IndexType >:: max () . index ())) }
};
}
