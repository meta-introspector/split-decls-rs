// Generated macro for impl_128 (impl)
macro_rules! Depcrate_recv_streamimpl_128 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "futures-io")] impl futures_io :: AsyncRead for RecvStream { fn poll_read (self : Pin < & mut Self > , cx : & mut Context , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { let mut buf = ReadBuf :: new (buf) ; ready ! (Self :: poll_read_buf (self . get_mut () , cx , & mut buf)) ? ; Poll :: Ready (Ok (buf . filled () . len ())) } }
};
}
