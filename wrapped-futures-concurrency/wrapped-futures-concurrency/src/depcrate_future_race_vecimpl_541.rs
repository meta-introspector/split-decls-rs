// Generated macro for impl_541 (impl)
macro_rules! Depcrate_future_race_vecimpl_541 {
() => {
// Module: crate::future::race::vec
// Provides: {"impl_541"}
// Dependencies: {}
impl < Fut > RaceTrait for Vec < Fut > where Fut : IntoFuture , { type Output = Fut :: Output ; type Future = Race < Fut :: IntoFuture > ; fn race (self) -> Self :: Future { Race { indexer : Indexer :: new (self . len ()) , futures : self . into_iter () . map (| fut | fut . into_future ()) . collect () , done : false , } } }
};
}
