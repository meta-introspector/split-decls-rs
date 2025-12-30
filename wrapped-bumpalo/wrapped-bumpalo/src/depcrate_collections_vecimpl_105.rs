// Generated macro for impl_105 (impl)
macro_rules! Depcrate_collections_vecimpl_105 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a , 'bump , T > IntoIterator for & 'a mut Vec < 'bump , T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> slice :: IterMut < 'a , T > { self . iter_mut () } }
};
}
