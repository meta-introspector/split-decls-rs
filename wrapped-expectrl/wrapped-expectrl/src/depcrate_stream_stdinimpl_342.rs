// Generated macro for impl_342 (impl)
macro_rules! Depcrate_stream_stdinimpl_342 {
() => {
// Module: crate::stream::stdin
// Provides: {"impl_342"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncRead for Stdin { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { AsyncRead :: poll_read (Pin :: new (& mut self . inner) , cx , buf) } }
};
}
