// Generated macro for sequential_update (function)
macro_rules! Depcrate_bit_setsequential_update {
() => {
// Module: crate::bit_set
// Provides: {"sequential_update"}
// Dependencies: {}
fn sequential_update < T : Idx > (mut self_update : impl FnMut (T) -> bool , it : impl Iterator < Item = T > ,) -> bool { it . fold (false , | changed , elem | self_update (elem) | changed) }
};
}
