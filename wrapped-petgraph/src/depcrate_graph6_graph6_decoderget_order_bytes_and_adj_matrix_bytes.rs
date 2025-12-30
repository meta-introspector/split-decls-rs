// Generated macro for get_order_bytes_and_adj_matrix_bytes (function)
macro_rules! Depcrate_graph6_graph6_decoderget_order_bytes_and_adj_matrix_bytes {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"get_order_bytes_and_adj_matrix_bytes"}
// Dependencies: {}
fn get_order_bytes_and_adj_matrix_bytes (graph6_representation : String) -> (Vec < usize > , Vec < usize >) { let bytes : Vec < usize > = graph6_representation . chars () . map (| c | (c as usize) - N) . collect () ; let mut order_bytes = vec ! [] ; let mut adj_matrix_bytes = vec ! [] ; let first_byte = * bytes . first () . unwrap () ; if first_byte == N { order_bytes . extend_from_slice (& bytes [1 ..= 3]) ; adj_matrix_bytes . extend_from_slice (& bytes [4 ..]) ; } else { order_bytes . push (first_byte) ; adj_matrix_bytes . extend_from_slice (& bytes [1 ..]) ; } ; (order_bytes , adj_matrix_bytes) }
};
}
