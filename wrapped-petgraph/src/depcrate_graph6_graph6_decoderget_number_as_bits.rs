// Generated macro for get_number_as_bits (function)
macro_rules! Depcrate_graph6_graph6_decoderget_number_as_bits {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"get_number_as_bits"}
// Dependencies: {}
fn get_number_as_bits (n : usize , bits_length : usize) -> Vec < u8 > { let mut bits = Vec :: new () ; for i in (0 .. bits_length) . rev () { bits . push (((n >> i) & 1) as u8) ; } bits }
};
}
