// Generated macro for impl_431 (impl)
macro_rules! Depcrate_future_select_allimpl_431 {
() => {
// Module: crate::future::select_all
// Provides: {"impl_431"}
// Dependencies: {}
impl < Fut : Future + Unpin > FromIterator < Fut > for SelectAll < Fut > { fn from_iter < T : IntoIterator < Item = Fut > > (iter : T) -> Self { select_all (iter) } }
};
}
