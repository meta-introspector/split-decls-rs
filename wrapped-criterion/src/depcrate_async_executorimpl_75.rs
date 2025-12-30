// Generated macro for impl_75 (impl)
macro_rules! Depcrate_async_executorimpl_75 {
() => {
// Module: crate::async_executor
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "async_smol")] impl AsyncExecutor for SmolExecutor { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { smol :: block_on (future) } }
};
}
