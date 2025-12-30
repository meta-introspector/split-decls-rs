// Generated macro for impl_73 (impl)
macro_rules! Depcrate_async_executorimpl_73 {
() => {
// Module: crate::async_executor
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg (feature = "async_futures")] impl AsyncExecutor for FuturesExecutor { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { futures :: executor :: block_on (future) } }
};
}
