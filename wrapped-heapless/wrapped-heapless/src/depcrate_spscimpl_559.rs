// Generated macro for impl_559 (impl)
macro_rules! Depcrate_spscimpl_559 {
() => {
// Module: crate::spsc
// Provides: {"impl_559"}
// Dependencies: {}
impl < 'a , T , S : Storage > IntoIterator for & 'a mut QueueInner < T , S > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
