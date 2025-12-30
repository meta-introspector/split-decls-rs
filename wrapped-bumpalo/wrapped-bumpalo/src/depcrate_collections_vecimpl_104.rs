// Generated macro for impl_104 (impl)
macro_rules! Depcrate_collections_vecimpl_104 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a , 'bump , T > IntoIterator for & 'a Vec < 'bump , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> slice :: Iter < 'a , T > { self . iter () } }
};
}
