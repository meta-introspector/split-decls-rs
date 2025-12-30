// Generated macro for impl_1808 (impl)
macro_rules! Depcrate_stream_select_allimpl_1808 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1808"}
// Dependencies: {}
impl < 'a , St : Stream + Unpin > IntoIterator for & 'a SelectAll < St > { type Item = & 'a St ; type IntoIter = Iter < 'a , St > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
