// Generated macro for impl_464 (impl)
macro_rules! Depcrate_future_try_join_allimpl_464 {
() => {
// Module: crate::future::try_join_all
// Provides: {"impl_464"}
// Dependencies: {}
impl < F > FromIterator < F > for TryJoinAll < F > where F : TryFuture , { fn from_iter < T : IntoIterator < Item = F > > (iter : T) -> Self { try_join_all (iter) } }
};
}
