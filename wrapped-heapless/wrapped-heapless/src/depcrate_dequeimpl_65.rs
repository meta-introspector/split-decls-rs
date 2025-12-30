// Generated macro for impl_65 (impl)
macro_rules! Depcrate_dequeimpl_65 {
() => {
// Module: crate::deque
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a , T , S : VecStorage < T > + ? Sized > IntoIterator for & 'a DequeInner < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
