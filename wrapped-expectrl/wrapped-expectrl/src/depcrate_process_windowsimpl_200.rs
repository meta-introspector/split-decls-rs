// Generated macro for impl_200 (impl)
macro_rules! Depcrate_process_windowsimpl_200 {
() => {
// Module: crate::process::windows
// Provides: {"impl_200"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncWrite for AsyncProcessStream { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { Pin :: new (& mut self . input) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . input) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . input) . poll_close (cx) } }
};
}
