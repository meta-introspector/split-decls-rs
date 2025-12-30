// Generated macro for to_linearized_matrix_position (function)
macro_rules! Depcrate_matrix_graphto_linearized_matrix_position {
() => {
// Module: crate::matrix_graph
// Provides: {"to_linearized_matrix_position"}
// Dependencies: {}
# [inline] fn to_linearized_matrix_position < Ty : EdgeType > (row : usize , column : usize , width : usize) -> usize { if Ty :: is_directed () { to_flat_square_matrix_position (row , column , width) } else { to_lower_triangular_matrix_position (row , column) } }
};
}
