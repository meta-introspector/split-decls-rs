// Generated macro for impl_564 (impl)
macro_rules! Depcrate_tableimpl_564 {
() => {
// Module: crate::table
// Provides: {"impl_564"}
// Dependencies: {}
impl < 'a , T , A > IntoIterator for & 'a mut HashTable < T , A > where A : Allocator , { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
