// Generated macro for impl_128 (impl)
macro_rules! Depcrate_sliceimpl_128 {
() => {
// Module: crate::slice
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'a , I : Idx , T > IntoIterator for & 'a mut IndexSlice < I , T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; # [inline] fn into_iter (self) -> slice :: IterMut < 'a , T > { self . raw . iter_mut () } }
};
}
