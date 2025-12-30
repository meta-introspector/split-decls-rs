// Generated macro for impl_134 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_134 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_134"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] impl < R : tokio_03_dep :: io :: AsyncRead > tokio_03_dep :: io :: AsyncRead for BufReader < R > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut tokio_03_dep :: io :: ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { if ! self . buf . has_remaining_mut () && buf . remaining () >= self . buf . len () { let res = ready ! (self . as_mut () . get_pin_mut () . poll_read (cx , buf)) ; self . discard_buffer () ; return Poll :: Ready (res) ; } let rem = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let amt = std :: cmp :: min (rem . len () , buf . remaining ()) ; buf . put_slice (& rem [.. amt]) ; self . consume (amt) ; Poll :: Ready (Ok (())) } }
};
}
