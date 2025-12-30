// Generated macro for num_words (function)
macro_rules! Depcrate_bit_setnum_words {
() => {
// Module: crate::bit_set
// Provides: {"num_words"}
// Dependencies: {}
# [inline] fn num_words < T : Idx > (domain_size : T) -> usize { domain_size . index () . div_ceil (WORD_BITS) }
};
}
