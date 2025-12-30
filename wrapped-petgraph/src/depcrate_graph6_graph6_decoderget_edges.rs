// Generated macro for get_edges (function)
macro_rules! Depcrate_graph6_graph6_decoderget_edges {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"get_edges"}
// Dependencies: {}
fn get_edges < Ix > (order : usize , adj_matrix_bits : Vec < u8 >) -> Vec < (Ix , Ix) > where Ix : IndexType , { let mut edges = vec ! [] ; let mut i = 0 ; for col in 1 .. order { for lin in 0 .. col { let is_adjacent = adj_matrix_bits [i] == 1 ; if is_adjacent { edges . push ((Ix :: new (lin) , Ix :: new (col))) ; } ; i += 1 ; } } edges }
};
}
