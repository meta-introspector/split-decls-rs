// Generated macro for word_index_and_mask (function)
macro_rules! Depcrate_bit_setword_index_and_mask {
() => {
// Module: crate::bit_set
// Provides: {"word_index_and_mask"}
// Dependencies: {}
# [inline] fn word_index_and_mask < T : Idx > (elem : T) -> (usize , Word) { let elem = elem . index () ; let word_index = elem / WORD_BITS ; let mask = 1 << (elem % WORD_BITS) ; (word_index , mask) }
};
}
