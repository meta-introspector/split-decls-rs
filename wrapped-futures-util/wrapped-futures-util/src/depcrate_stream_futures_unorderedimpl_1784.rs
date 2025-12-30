// Generated macro for impl_1784 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1784 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1784"}
// Dependencies: {}
impl < Fut : Unpin > IntoIterator for FuturesUnordered < Fut > { type Item = Fut ; type IntoIter = IntoIter < Fut > ; fn into_iter (mut self) -> Self :: IntoIter { let task = * self . head_all . get_mut () ; let len = if task . is_null () { 0 } else { unsafe { * (* task) . len_all . get () } } ; IntoIter { len , inner : self } } }
};
}
