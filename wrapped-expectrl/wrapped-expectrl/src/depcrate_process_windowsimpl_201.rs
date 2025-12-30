// Generated macro for impl_201 (impl)
macro_rules! Depcrate_process_windowsimpl_201 {
() => {
// Module: crate::process::windows
// Provides: {"impl_201"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncRead for AsyncProcessStream { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { Pin :: new (& mut self . output) . poll_read (cx , buf) } }
};
}
