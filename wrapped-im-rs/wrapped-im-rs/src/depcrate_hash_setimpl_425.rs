// Generated macro for impl_425 (impl)
macro_rules! Depcrate_hash_setimpl_425 {
() => {
// Module: crate::hash::set
// Provides: {"impl_425"}
// Dependencies: {}
impl < A , S > IntoIterator for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher , { type Item = A ; type IntoIter = ConsumingIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { ConsumingIter { it : NodeDrain :: new (& self . pool . 0 , self . root , self . size) , } } }
};
}
