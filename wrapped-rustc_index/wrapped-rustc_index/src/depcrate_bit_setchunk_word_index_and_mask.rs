// Generated macro for chunk_word_index_and_mask (function)
macro_rules! Depcrate_bit_setchunk_word_index_and_mask {
() => {
// Module: crate::bit_set
// Provides: {"chunk_word_index_and_mask"}
// Dependencies: {}
# [inline] fn chunk_word_index_and_mask < T : Idx > (elem : T) -> (usize , Word) { let chunk_elem = elem . index () % CHUNK_BITS ; word_index_and_mask (chunk_elem) }
};
}
