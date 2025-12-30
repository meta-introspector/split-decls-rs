// Generated macro for impl_403 (impl)
macro_rules! Depcrate_future_join_allimpl_403 {
() => {
// Module: crate::future::join_all
// Provides: {"impl_403"}
// Dependencies: {}
impl < F : Future > FromIterator < F > for JoinAll < F > { fn from_iter < T : IntoIterator < Item = F > > (iter : T) -> Self { join_all (iter) } }
};
}
