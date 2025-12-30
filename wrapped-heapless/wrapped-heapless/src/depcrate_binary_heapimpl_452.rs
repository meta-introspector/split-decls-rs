// Generated macro for impl_452 (impl)
macro_rules! Depcrate_binary_heapimpl_452 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_452"}
// Dependencies: {}
impl < 'a , T , K , S > IntoIterator for & 'a BinaryHeapInner < T , K , S > where K : Kind , T : Ord , S : VecStorage < T > + ? Sized , { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
