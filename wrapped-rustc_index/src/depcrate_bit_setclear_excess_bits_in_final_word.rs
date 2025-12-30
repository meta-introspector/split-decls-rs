// Generated macro for clear_excess_bits_in_final_word (function)
macro_rules! Depcrate_bit_setclear_excess_bits_in_final_word {
() => {
// Module: crate::bit_set
// Provides: {"clear_excess_bits_in_final_word"}
// Dependencies: {}
fn clear_excess_bits_in_final_word (domain_size : usize , words : & mut [Word]) { let num_bits_in_final_word = domain_size % WORD_BITS ; if num_bits_in_final_word > 0 { let mask = (1 << num_bits_in_final_word) - 1 ; words [words . len () - 1] &= mask ; } }
};
}
