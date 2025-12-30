// Generated macro for impl_437 (impl)
macro_rules! Depcrate_future_join_arrayimpl_437 {
() => {
// Module: crate::future::join::array
// Provides: {"impl_437"}
// Dependencies: {}
impl < Fut , const N : usize > JoinTrait for [Fut ; N] where Fut : IntoFuture , { type Output = [Fut :: Output ; N] ; type Future = Join < Fut :: IntoFuture , N > ; # [inline] fn join (self) -> Self :: Future { Join :: new (self . map (IntoFuture :: into_future)) } }
};
}
