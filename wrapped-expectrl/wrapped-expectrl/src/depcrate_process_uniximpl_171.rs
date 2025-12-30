// Generated macro for impl_171 (impl)
macro_rules! Depcrate_process_uniximpl_171 {
() => {
// Module: crate::process::unix
// Provides: {"impl_171"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncRead for AsyncPtyStream { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { Pin :: new (& mut self . stream) . poll_read (cx , buf) } }
};
}
