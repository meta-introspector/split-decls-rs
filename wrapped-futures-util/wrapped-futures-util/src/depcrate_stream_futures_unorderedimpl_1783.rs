// Generated macro for impl_1783 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1783 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1783"}
// Dependencies: {}
impl < 'a , Fut : Unpin > IntoIterator for & 'a mut FuturesUnordered < Fut > { type Item = & 'a mut Fut ; type IntoIter = IterMut < 'a , Fut > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
