// Generated macro for impl_197 (impl)
macro_rules! Depcrate_process_windowsimpl_197 {
() => {
// Module: crate::process::windows
// Provides: {"impl_197"}
// Dependencies: {}
# [cfg (feature = "async")] impl IntoAsyncStream for ProcessStream { type AsyncStream = AsyncProcessStream ; fn into_async_stream (self) -> Result < Self :: AsyncStream > { AsyncProcessStream :: new (self) } }
};
}
