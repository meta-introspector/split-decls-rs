// Generated macro for impl_658 (impl)
macro_rules! Depcrate_future_try_join_arrayimpl_658 {
() => {
// Module: crate::future::try_join::array
// Provides: {"impl_658"}
// Dependencies: {}
impl < Fut , T , E , const N : usize > TryJoinTrait for [Fut ; N] where Fut : IntoFuture < Output = Result < T , E > > , { type Output = [T ; N] ; type Error = E ; type Future = TryJoin < Fut :: IntoFuture , T , E , N > ; fn try_join (self) -> Self :: Future { TryJoin :: new (self . map (IntoFuture :: into_future)) } }
};
}
