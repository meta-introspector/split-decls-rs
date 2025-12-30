// Generated macro for impl_96 (impl)
macro_rules! Depcrate_ordered_setimpl_96 {
() => {
// Module: crate::ordered_set
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a OrderedSet < T > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
