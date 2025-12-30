// Generated macro for extend_linearized_matrix (function)
macro_rules! Depcrate_matrix_graphextend_linearized_matrix {
() => {
// Module: crate::matrix_graph
// Provides: {"extend_linearized_matrix"}
// Dependencies: {}
# [inline] fn extend_linearized_matrix < Ty : EdgeType , T : Default > (node_adjacencies : & mut Vec < T > , old_node_capacity : usize , new_capacity : usize , exact : bool ,) -> usize { if old_node_capacity >= new_capacity { return old_node_capacity ; } if Ty :: is_directed () { extend_flat_square_matrix (node_adjacencies , old_node_capacity , new_capacity , exact) } else { extend_lower_triangular_matrix (node_adjacencies , new_capacity) } }
};
}
