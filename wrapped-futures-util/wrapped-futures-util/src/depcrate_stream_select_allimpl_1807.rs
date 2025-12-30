// Generated macro for impl_1807 (impl)
macro_rules! Depcrate_stream_select_allimpl_1807 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1807"}
// Dependencies: {}
impl < St : Stream + Unpin > IntoIterator for SelectAll < St > { type Item = St ; type IntoIter = IntoIter < St > ; fn into_iter (self) -> Self :: IntoIter { IntoIter (self . inner . into_iter ()) } }
};
}
