// Generated macro for max_bit (function)
macro_rules! Depcrate_bit_setmax_bit {
() => {
// Module: crate::bit_set
// Provides: {"max_bit"}
// Dependencies: {}
# [inline] fn max_bit (word : Word) -> usize { WORD_BITS - 1 - word . leading_zeros () as usize }
};
}
