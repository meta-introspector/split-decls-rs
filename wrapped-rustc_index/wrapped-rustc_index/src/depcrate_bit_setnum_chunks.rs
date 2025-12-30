// Generated macro for num_chunks (function)
macro_rules! Depcrate_bit_setnum_chunks {
() => {
// Module: crate::bit_set
// Provides: {"num_chunks"}
// Dependencies: {}
# [inline] fn num_chunks < T : Idx > (domain_size : T) -> usize { assert ! (domain_size . index () > 0) ; domain_size . index () . div_ceil (CHUNK_BITS) }
};
}
