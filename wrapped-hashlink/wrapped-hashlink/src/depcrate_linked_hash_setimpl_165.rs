// Generated macro for impl_165 (impl)
macro_rules! Depcrate_linked_hash_setimpl_165 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a , T , S > IntoIterator for & 'a LinkedHashSet < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; # [inline] fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
