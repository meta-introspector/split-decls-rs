// Generated macro for impl_563 (impl)
macro_rules! Depcrate_tableimpl_563 {
() => {
// Module: crate::table
// Provides: {"impl_563"}
// Dependencies: {}
impl < 'a , T , A > IntoIterator for & 'a HashTable < T , A > where A : Allocator , { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
