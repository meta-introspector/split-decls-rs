// Generated macro for get_adj_matrix_upper_diagonal_as_bits (function)
macro_rules! Depcrate_graph6_graph6_encoderget_adj_matrix_upper_diagonal_as_bits {
() => {
// Module: crate::graph6::graph6_encoder
// Provides: {"get_adj_matrix_upper_diagonal_as_bits"}
// Dependencies: {}
fn get_adj_matrix_upper_diagonal_as_bits < G > (graph : G) -> (usize , Vec < usize >) where G : GetAdjacencyMatrix + IntoNodeIdentifiers , { let node_ids_iter = graph . node_identifiers () ; let mut node_ids_vec = vec ! [] ; let adj_matrix = graph . adjacency_matrix () ; let mut bits = vec ! [] ; let mut n = 0 ; for node_id in node_ids_iter { node_ids_vec . push (node_id) ; for i in 1 ..= n { let is_adjacent : bool = graph . is_adjacent (& adj_matrix , node_ids_vec [i - 1] , node_ids_vec [n]) ; bits . push (if is_adjacent { 1 } else { 0 }) ; } n += 1 ; } (n , bits) }
};
}
