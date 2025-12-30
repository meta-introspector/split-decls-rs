// Generated macro for impl_40 (impl)
macro_rules! Depcrate_channelimpl_40 {
() => {
// Module: crate::channel
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a Receiver < T > { type Item = T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
