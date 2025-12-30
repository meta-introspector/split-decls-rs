// Generated macro for impl_127 (impl)
macro_rules! Depcrate_sliceimpl_127 {
() => {
// Module: crate::slice
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a , I : Idx , T > IntoIterator for & 'a IndexSlice < I , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline] fn into_iter (self) -> slice :: Iter < 'a , T > { self . raw . iter () } }
};
}
