// Generated macro for FutureOrOutput (enum)
macro_rules! Depcrate_future_future_sharedFutureOrOutput {
() => {
// Module: crate::future::future::shared
// Provides: {"FutureOrOutput"}
// Dependencies: {}
enum FutureOrOutput < Fut : Future > { Future (Fut) , Output (Fut :: Output) , }
};
}
