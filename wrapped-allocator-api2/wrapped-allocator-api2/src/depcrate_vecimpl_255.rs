// Generated macro for impl_255 (impl)
macro_rules! Depcrate_vecimpl_255 {
() => {
// Module: crate::vec
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a , T , A : Allocator > IntoIterator for & 'a mut Vec < T , A > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
