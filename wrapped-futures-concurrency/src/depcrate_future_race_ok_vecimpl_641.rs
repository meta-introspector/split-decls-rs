// Generated macro for impl_641 (impl)
macro_rules! Depcrate_future_race_ok_vecimpl_641 {
() => {
// Module: crate::future::race_ok::vec
// Provides: {"impl_641"}
// Dependencies: {}
impl < Fut , T , E > RaceOkTrait for Vec < Fut > where Fut : IntoFuture < Output = Result < T , E > > , { type Output = T ; type Error = AggregateError < E > ; type Future = RaceOk < Fut :: IntoFuture , T , E > ; fn race_ok (self) -> Self :: Future { let elems : Box < [_] > = self . into_iter () . map (| fut | MaybeDone :: new (fut . into_future ())) . collect () ; RaceOk { elems : elems . into () , } } }
};
}
