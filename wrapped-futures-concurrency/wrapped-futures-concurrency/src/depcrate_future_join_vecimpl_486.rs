// Generated macro for impl_486 (impl)
macro_rules! Depcrate_future_join_vecimpl_486 {
() => {
// Module: crate::future::join::vec
// Provides: {"impl_486"}
// Dependencies: {}
impl < Fut > JoinTrait for Vec < Fut > where Fut : IntoFuture , { type Output = Vec < Fut :: Output > ; type Future = Join < Fut :: IntoFuture > ; fn join (self) -> Self :: Future { Join :: new (self . into_iter () . map (IntoFuture :: into_future) . collect ()) } }
};
}
