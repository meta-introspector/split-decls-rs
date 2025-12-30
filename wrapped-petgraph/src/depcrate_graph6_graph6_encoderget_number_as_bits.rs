// Generated macro for get_number_as_bits (function)
macro_rules! Depcrate_graph6_graph6_encoderget_number_as_bits {
() => {
// Module: crate::graph6::graph6_encoder
// Provides: {"get_number_as_bits"}
// Dependencies: {}
fn get_number_as_bits (n : usize , bits_length : usize) -> Vec < usize > { let mut bits = Vec :: new () ; for i in (0 .. bits_length) . rev () { bits . push ((n >> i) & 1) ; } bits }
};
}
