// Generated macro for impl_81 (impl)
macro_rules! Depcrate_async_executorimpl_81 {
() => {
// Module: crate::async_executor
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (feature = "async_std")] impl AsyncExecutor for AsyncStdExecutor { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { async_std :: task :: block_on (future) } }
};
}
