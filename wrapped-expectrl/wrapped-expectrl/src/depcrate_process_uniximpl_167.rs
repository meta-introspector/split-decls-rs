// Generated macro for impl_167 (impl)
macro_rules! Depcrate_process_uniximpl_167 {
() => {
// Module: crate::process::unix
// Provides: {"impl_167"}
// Dependencies: {}
# [cfg (feature = "async")] impl IntoAsyncStream for PtyStream { type AsyncStream = AsyncPtyStream ; fn into_async_stream (self) -> Result < Self :: AsyncStream > { AsyncPtyStream :: new (self) } }
};
}
