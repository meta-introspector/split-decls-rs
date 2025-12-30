// Generated macro for impl_131 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_131 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_131"}
// Dependencies: {}
# [cfg (feature = "tokio-02")] impl < R : tokio_02_dep :: io :: AsyncRead > tokio_02_dep :: io :: AsyncRead for BufReader < R > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { use tokio_02_dep :: io :: AsyncBufRead ; if ! self . buf . has_remaining_mut () && buf . len () >= self . buf . len () { let res = ready ! (self . as_mut () . get_pin_mut () . poll_read (cx , buf)) ; self . discard_buffer () ; return Poll :: Ready (res) ; } let mut rem = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let nread = rem . read (buf) ? ; self . consume (nread) ; Poll :: Ready (Ok (nread)) } unsafe fn prepare_uninitialized_buffer (& self , buf : & mut [MaybeUninit < u8 >]) -> bool { self . inner . prepare_uninitialized_buffer (buf) } }
};
}
