// Generated macro for invalid_hole_err (function)
macro_rules! Depcrate_graph_impl_serializationinvalid_hole_err {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"invalid_hole_err"}
// Dependencies: {}
pub fn invalid_hole_err < E > (node_index : usize) -> E where E : Error , { E :: custom (format_args ! ("invalid value: node hole `{node_index}` is not allowed." ,)) }
};
}
