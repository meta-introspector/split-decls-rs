// Generated macro for extend_lower_triangular_matrix (function)
macro_rules! Depcrate_matrix_graphextend_lower_triangular_matrix {
() => {
// Module: crate::matrix_graph
// Provides: {"extend_lower_triangular_matrix"}
// Dependencies: {}
# [inline] fn extend_lower_triangular_matrix < T : Default > (node_adjacencies : & mut Vec < T > , new_capacity : usize ,) -> usize { let max_node = new_capacity - 1 ; let max_pos = to_lower_triangular_matrix_position (max_node , max_node) ; ensure_len (node_adjacencies , max_pos + 1) ; new_capacity }
};
}
