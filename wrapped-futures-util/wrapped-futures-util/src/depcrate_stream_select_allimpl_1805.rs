// Generated macro for impl_1805 (impl)
macro_rules! Depcrate_stream_select_allimpl_1805 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1805"}
// Dependencies: {}
impl < St : Stream + Unpin > FromIterator < St > for SelectAll < St > { fn from_iter < T : IntoIterator < Item = St > > (iter : T) -> Self { select_all (iter) } }
};
}
