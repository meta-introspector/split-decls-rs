// Generated macro for impl_78 (impl)
macro_rules! Depcrate_async_executorimpl_78 {
() => {
// Module: crate::async_executor
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (feature = "async_tokio")] impl AsyncExecutor for tokio :: runtime :: Handle { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { self . block_on (future) } }
};
}
