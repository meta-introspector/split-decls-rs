// Generated macro for impl_1809 (impl)
macro_rules! Depcrate_stream_select_allimpl_1809 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1809"}
// Dependencies: {}
impl < 'a , St : Stream + Unpin > IntoIterator for & 'a mut SelectAll < St > { type Item = & 'a mut St ; type IntoIter = IterMut < 'a , St > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
