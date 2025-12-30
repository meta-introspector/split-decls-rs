// Generated macro for impl_491 (impl)
macro_rules! Depcrate_future_select_okimpl_491 {
() => {
// Module: crate::future::select_ok
// Provides: {"impl_491"}
// Dependencies: {}
impl < Fut : TryFuture + Unpin > FromIterator < Fut > for SelectOk < Fut > { fn from_iter < T : IntoIterator < Item = Fut > > (iter : T) -> Self { select_ok (iter) } }
};
}
