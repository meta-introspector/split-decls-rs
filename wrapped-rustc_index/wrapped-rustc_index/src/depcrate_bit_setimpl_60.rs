// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bit_setimpl_60 {
() => {
// Module: crate::bit_set
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : Idx > GrowableBitSet < T > { # [doc = " Ensure that the set can hold at least `min_domain_size` elements."] pub fn ensure (& mut self , min_domain_size : usize) { if self . bit_set . domain_size < min_domain_size { self . bit_set . domain_size = min_domain_size ; } let min_num_words = num_words (min_domain_size) ; if self . bit_set . words . len () < min_num_words { self . bit_set . words . resize (min_num_words , 0) } } pub fn new_empty () -> GrowableBitSet < T > { GrowableBitSet { bit_set : DenseBitSet :: new_empty (0) } } pub fn with_capacity (capacity : usize) -> GrowableBitSet < T > { GrowableBitSet { bit_set : DenseBitSet :: new_empty (capacity) } } # [doc = " Returns `true` if the set has changed."] # [inline] pub fn insert (& mut self , elem : T) -> bool { self . ensure (elem . index () + 1) ; self . bit_set . insert (elem) } # [doc = " Returns `true` if the set has changed."] # [inline] pub fn remove (& mut self , elem : T) -> bool { self . ensure (elem . index () + 1) ; self . bit_set . remove (elem) } # [inline] pub fn is_empty (& self) -> bool { self . bit_set . is_empty () } # [inline] pub fn contains (& self , elem : T) -> bool { let (word_index , mask) = word_index_and_mask (elem) ; self . bit_set . words . get (word_index) . is_some_and (| word | (word & mask) != 0) } # [inline] pub fn iter (& self) -> BitIter < '_ , T > { self . bit_set . iter () } # [inline] pub fn len (& self) -> usize { self . bit_set . count () } }
};
}
