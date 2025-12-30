// Generated macro for bytes_vector_to_bits_vector (function)
macro_rules! Depcrate_graph6_graph6_decoderbytes_vector_to_bits_vector {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"bytes_vector_to_bits_vector"}
// Dependencies: {}
fn bytes_vector_to_bits_vector (bytes : Vec < usize >) -> Vec < u8 > { bytes . iter () . flat_map (| & byte | get_number_as_bits (byte , 6)) . collect () }
};
}
