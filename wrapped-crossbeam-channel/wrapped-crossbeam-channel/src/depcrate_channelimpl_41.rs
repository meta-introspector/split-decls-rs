// Generated macro for impl_41 (impl)
macro_rules! Depcrate_channelimpl_41 {
() => {
// Module: crate::channel
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > IntoIterator for Receiver < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { receiver : self } } }
};
}
