// Generated macro for impl_79 (impl)
macro_rules! Depcrate_async_executorimpl_79 {
() => {
// Module: crate::async_executor
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "async_tokio")] impl AsyncExecutor for & tokio :: runtime :: Handle { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { (* self) . block_on (future) } }
};
}
