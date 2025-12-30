// Generated macro for impl_87 (impl)
macro_rules! Depcrate_slice_revimpl_87 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a mut RevSlice < T > { type Item = & 'a mut T ; type IntoIter = Rev < IterMut < 'a , T > > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . iter_mut () . rev () } }
};
}
