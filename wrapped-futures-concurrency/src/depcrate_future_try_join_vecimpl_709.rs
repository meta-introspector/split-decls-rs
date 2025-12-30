// Generated macro for impl_709 (impl)
macro_rules! Depcrate_future_try_join_vecimpl_709 {
() => {
// Module: crate::future::try_join::vec
// Provides: {"impl_709"}
// Dependencies: {}
impl < Fut , T , E > TryJoinTrait for Vec < Fut > where Fut : IntoFuture < Output = Result < T , E > > , { type Output = Vec < T > ; type Error = E ; type Future = TryJoin < Fut :: IntoFuture , T , E > ; fn try_join (self) -> Self :: Future { TryJoin :: new (self . into_iter () . map (IntoFuture :: into_future) . collect ()) } }
};
}
