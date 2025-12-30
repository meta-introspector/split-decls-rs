// Generated macro for impl_708 (impl)
macro_rules! Depcrate_future_try_join_vecimpl_708 {
() => {
// Module: crate::future::try_join::vec
// Provides: {"impl_708"}
// Dependencies: {}
impl < Fut , T , E > TryJoin < Fut , T , E > where Fut : Future < Output = Result < T , E > > , { # [inline] pub (crate) fn new (futures : Vec < Fut >) -> Self { let len = futures . len () ; Self { consumed : false , pending : len , items : OutputVec :: uninit (len) , wakers : WakerVec :: new (len) , state : PollVec :: new_pending (len) , futures : FutureVec :: new (futures) , } } }
};
}
