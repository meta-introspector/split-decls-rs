// Generated macro for impl_153 (impl)
macro_rules! Depcrate_vecimpl_153 {
() => {
// Module: crate::vec
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a , I : Idx , T > IntoIterator for & 'a mut IndexVec < I , T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; # [inline] fn into_iter (self) -> slice :: IterMut < 'a , T > { self . iter_mut () } }
};
}
