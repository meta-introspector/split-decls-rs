// Generated macro for to_lower_triangular_matrix_position (function)
macro_rules! Depcrate_matrix_graphto_lower_triangular_matrix_position {
() => {
// Module: crate::matrix_graph
// Provides: {"to_lower_triangular_matrix_position"}
// Dependencies: {}
# [inline] fn to_lower_triangular_matrix_position (row : usize , column : usize) -> usize { let (row , column) = if row > column { (row , column) } else { (column , row) } ; (row * (row + 1)) / 2 + column }
};
}
