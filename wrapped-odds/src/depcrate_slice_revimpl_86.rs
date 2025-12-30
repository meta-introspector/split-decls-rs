// Generated macro for impl_86 (impl)
macro_rules! Depcrate_slice_revimpl_86 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a RevSlice < T > { type Item = & 'a T ; type IntoIter = Rev < Iter < 'a , T > > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . iter () . rev () } }
};
}
