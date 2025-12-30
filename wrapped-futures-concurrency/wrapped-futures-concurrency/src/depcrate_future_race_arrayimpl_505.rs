// Generated macro for impl_505 (impl)
macro_rules! Depcrate_future_race_arrayimpl_505 {
() => {
// Module: crate::future::race::array
// Provides: {"impl_505"}
// Dependencies: {}
impl < Fut , const N : usize > RaceTrait for [Fut ; N] where Fut : IntoFuture , { type Output = Fut :: Output ; type Future = Race < Fut :: IntoFuture , N > ; fn race (self) -> Self :: Future { Race { futures : self . map (| fut | fut . into_future ()) , indexer : Indexer :: new (N) , done : false , } } }
};
}
