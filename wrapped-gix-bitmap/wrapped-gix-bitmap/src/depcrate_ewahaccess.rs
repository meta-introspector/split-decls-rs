// Generated macro for access (module)
macro_rules! Depcrate_ewahaccess {
() => {
// Module: crate::ewah
// Provides: {"access"}
// Dependencies: {}
mod access { use super :: Vec ; impl Vec { # [doc = " Call `f(index)` for each bit that is true, given the index of the bit that identifies it uniquely within the bit array."] # [doc = " If `f` returns `None` the iteration will be stopped and `None` is returned."] # [doc = ""] # [doc = " The index is sequential like in any other vector."] pub fn for_each_set_bit (& self , mut f : impl FnMut (usize) -> Option < () >) -> Option < () > { let mut index = 0usize ; let mut iter = self . bits . iter () ; while let Some (word) = iter . next () { if rlw_runbit_is_set (word) { let len = rlw_running_len_bits (word) ; for _ in 0 .. len { f (index) ? ; index += 1 ; } } else { index += usize :: try_from (rlw_running_len_bits (word)) . ok () ? ; } for _ in 0 .. rlw_literal_words (word) { let word = iter . next () . expect ("BUG: ran out of words while going through uncompressed portion") ; for bit_index in 0 .. 64 { if word & (1 << bit_index) != 0 { f (index) ? ; } index += 1 ; } } } Some (()) } # [doc = " The amount of bits we are currently holding."] pub fn num_bits (& self) -> usize { self . num_bits . try_into () . expect ("we are not on 16 bit systems") } } # [inline] fn rlw_running_len_bits (w : & u64) -> u64 { rlw_running_len (w) * 64 } # [inline] fn rlw_running_len (w : & u64) -> u64 { (w >> 1) & RLW_LARGEST_RUNNING_COUNT } # [inline] fn rlw_literal_words (w : & u64) -> u64 { w >> (1 + RLW_RUNNING_BITS) } # [inline] fn rlw_runbit_is_set (w : & u64) -> bool { w & 1 == 1 } const RLW_RUNNING_BITS : u64 = 4 * 8 ; const RLW_LARGEST_RUNNING_COUNT : u64 = (1 << RLW_RUNNING_BITS) - 1 ; }
};
}
