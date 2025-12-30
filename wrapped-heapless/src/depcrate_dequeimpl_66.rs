// Generated macro for impl_66 (impl)
macro_rules! Depcrate_dequeimpl_66 {
() => {
// Module: crate::deque
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a , T , S : VecStorage < T > + ? Sized > IntoIterator for & 'a mut DequeInner < T , S > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
