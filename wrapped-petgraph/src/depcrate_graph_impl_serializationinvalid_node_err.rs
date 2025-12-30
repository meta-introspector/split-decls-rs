// Generated macro for invalid_node_err (function)
macro_rules! Depcrate_graph_impl_serializationinvalid_node_err {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"invalid_node_err"}
// Dependencies: {}
pub fn invalid_node_err < E > (node_index : usize , len : usize) -> E where E : Error , { E :: custom (format_args ! ("invalid value: node index `{node_index}` does not exist in graph \
         with node bound {len}" ,)) }
};
}
