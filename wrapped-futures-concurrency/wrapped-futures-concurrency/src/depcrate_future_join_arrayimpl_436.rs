// Generated macro for impl_436 (impl)
macro_rules! Depcrate_future_join_arrayimpl_436 {
() => {
// Module: crate::future::join::array
// Provides: {"impl_436"}
// Dependencies: {}
impl < Fut , const N : usize > Join < Fut , N > where Fut : Future , { # [inline] pub (crate) fn new (futures : [Fut ; N]) -> Self { Join { consumed : false , pending : N , items : OutputArray :: uninit () , wakers : WakerArray :: new () , state : PollArray :: new_pending () , futures : FutureArray :: new (futures) , } } }
};
}
