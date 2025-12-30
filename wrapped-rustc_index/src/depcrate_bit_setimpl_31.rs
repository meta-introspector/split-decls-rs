// Generated macro for impl_31 (impl)
macro_rules! Depcrate_bit_setimpl_31 {
() => {
// Module: crate::bit_set
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , T : Idx > BitIter < 'a , T > { # [inline] fn new (words : & 'a [Word]) -> BitIter < 'a , T > { BitIter { word : 0 , offset : usize :: MAX - (WORD_BITS - 1) , iter : words . iter () , marker : PhantomData , } } }
};
}
