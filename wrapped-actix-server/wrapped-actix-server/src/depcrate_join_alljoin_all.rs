// Generated macro for join_all (function)
macro_rules! Depcrate_join_alljoin_all {
() => {
// Module: crate::join_all
// Provides: {"join_all"}
// Dependencies: {}
pub (crate) fn join_all < T > (fut : Vec < impl Future < Output = T > + Send + 'static >) -> JoinAll < T > { let fut = fut . into_iter () . map (| f | JoinFuture :: Future (Box :: pin (f))) . collect () ; JoinAll { fut } }
};
}
