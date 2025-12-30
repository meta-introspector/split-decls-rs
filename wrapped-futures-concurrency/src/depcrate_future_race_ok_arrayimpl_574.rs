// Generated macro for impl_574 (impl)
macro_rules! Depcrate_future_race_ok_arrayimpl_574 {
() => {
// Module: crate::future::race_ok::array
// Provides: {"impl_574"}
// Dependencies: {}
impl < Fut , T , E , const N : usize > RaceOkTrait for [Fut ; N] where Fut : IntoFuture < Output = Result < T , E > > , { type Output = T ; type Error = AggregateError < E , N > ; type Future = RaceOk < Fut :: IntoFuture , T , E , N > ; fn race_ok (self) -> Self :: Future { RaceOk { futures : self . map (| fut | fut . into_future ()) , errors : array :: from_fn (| _ | MaybeUninit :: uninit ()) , error_states : PollArray :: new_pending () , completed : 0 , } } }
};
}
