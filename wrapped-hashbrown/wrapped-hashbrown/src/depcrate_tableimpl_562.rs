// Generated macro for impl_562 (impl)
macro_rules! Depcrate_tableimpl_562 {
() => {
// Module: crate::table
// Provides: {"impl_562"}
// Dependencies: {}
impl < T , A > IntoIterator for HashTable < T , A > where A : Allocator , { type Item = T ; type IntoIter = IntoIter < T , A > ; fn into_iter (self) -> IntoIter < T , A > { IntoIter { inner : self . raw . into_iter () , } } }
};
}
