// Generated macro for impl_657 (impl)
macro_rules! Depcrate_future_try_join_arrayimpl_657 {
() => {
// Module: crate::future::try_join::array
// Provides: {"impl_657"}
// Dependencies: {}
impl < Fut , T , E , const N : usize > TryJoin < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > , { # [inline] pub (crate) fn new (futures : [Fut ; N]) -> Self { Self { consumed : false , pending : N , items : OutputArray :: uninit () , wakers : WakerArray :: new () , state : PollArray :: new_pending () , futures : FutureArray :: new (futures) , } } }
};
}
