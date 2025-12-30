// Generated macro for impl_1782 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1782 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1782"}
// Dependencies: {}
impl < 'a , Fut : Unpin > IntoIterator for & 'a FuturesUnordered < Fut > { type Item = & 'a Fut ; type IntoIter = Iter < 'a , Fut > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
