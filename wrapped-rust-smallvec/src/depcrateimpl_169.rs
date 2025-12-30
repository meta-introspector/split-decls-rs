// Generated macro for impl_169 (impl)
macro_rules! Depcrateimpl_169 {
() => {
// Module: crate
// Provides: {"impl_169"}
// Dependencies: {}
impl < T , const N : usize > IntoIterator for SmallVec < T , N > { type IntoIter = IntoIter < T , N > ; type Item = T ; fn into_iter (self) -> Self :: IntoIter { unsafe { let this = ManuallyDrop :: new (self) ; IntoIter { raw : (& this . raw as * const RawSmallVec < T , N >) . read () , begin : 0 , end : this . len , _marker : PhantomData , } } } }
};
}
