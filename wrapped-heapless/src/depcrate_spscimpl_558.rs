// Generated macro for impl_558 (impl)
macro_rules! Depcrate_spscimpl_558 {
() => {
// Module: crate::spsc
// Provides: {"impl_558"}
// Dependencies: {}
impl < 'a , T , S : Storage > IntoIterator for & 'a QueueInner < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
