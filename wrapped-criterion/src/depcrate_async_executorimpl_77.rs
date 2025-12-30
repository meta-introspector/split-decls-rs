// Generated macro for impl_77 (impl)
macro_rules! Depcrate_async_executorimpl_77 {
() => {
// Module: crate::async_executor
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "async_tokio")] impl AsyncExecutor for & tokio :: runtime :: Runtime { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { (* self) . block_on (future) } }
};
}
