// Generated macro for impl_57 (impl)
macro_rules! Depcrate_bit_setimpl_57 {
() => {
// Module: crate::bit_set
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , T : Idx > Iterator for MixedBitIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { match self { MixedBitIter :: Small (iter) => iter . next () , MixedBitIter :: Large (iter) => iter . next () , } } }
};
}
