// Generated macro for impl_485 (impl)
macro_rules! Depcrate_future_join_vecimpl_485 {
() => {
// Module: crate::future::join::vec
// Provides: {"impl_485"}
// Dependencies: {}
impl < Fut > Join < Fut > where Fut : Future , { pub (crate) fn new (futures : Vec < Fut >) -> Self { let len = futures . len () ; Join { consumed : false , pending : len , items : OutputVec :: uninit (len) , wakers : WakerVec :: new (len) , state : PollVec :: new_pending (len) , futures : FutureVec :: new (futures) , } } }
};
}
