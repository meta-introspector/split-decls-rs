// Generated macro for impl_331 (impl)
macro_rules! Depcrate_stream_logimpl_331 {
() => {
// Module: crate::stream::log
// Provides: {"impl_331"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S : AsyncRead + Unpin , W : Write + Unpin > AsyncRead for LogStream < S , W > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let result = Pin :: new (& mut self . stream) . poll_read (cx , buf) ; if let Poll :: Ready (Ok (n)) = & result { self . log_read (& buf [.. * n]) ; } result } }
};
}
