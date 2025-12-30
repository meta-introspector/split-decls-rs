// Generated macro for impl_170 (impl)
macro_rules! Depcrate_process_uniximpl_170 {
() => {
// Module: crate::process::unix
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncWrite for AsyncPtyStream { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { Pin :: new (& mut self . stream) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . stream) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . stream) . poll_close (cx) } }
};
}
