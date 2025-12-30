// Generated macro for impl_32 (impl)
macro_rules! Depcrate_bit_setimpl_32 {
() => {
// Module: crate::bit_set
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a , T : Idx > Iterator for BitIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { loop { if self . word != 0 { let bit_pos = self . word . trailing_zeros () as usize ; self . word ^= 1 << bit_pos ; return Some (T :: new (bit_pos + self . offset)) ; } self . word = * self . iter . next () ? ; self . offset = self . offset . wrapping_add (WORD_BITS) ; } } }
};
}
