// Generated macro for impl_1806 (impl)
macro_rules! Depcrate_stream_select_allimpl_1806 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1806"}
// Dependencies: {}
impl < St : Stream + Unpin > Extend < St > for SelectAll < St > { fn extend < T : IntoIterator < Item = St > > (& mut self , iter : T) { for st in iter { self . push (st) } } }
};
}
