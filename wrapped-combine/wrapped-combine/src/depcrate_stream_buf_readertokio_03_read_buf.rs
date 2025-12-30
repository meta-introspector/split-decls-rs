// Generated macro for tokio_03_read_buf (function)
macro_rules! Depcrate_stream_buf_readertokio_03_read_buf {
() => {
// Module: crate::stream::buf_reader
// Provides: {"tokio_03_read_buf"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] fn tokio_03_read_buf (cx : & mut Context < '_ > , read : Pin < & mut impl tokio_03_dep :: io :: AsyncRead > , bs : & mut bytes :: BytesMut ,) -> Poll < io :: Result < usize > > { if ! bs . has_remaining_mut () { bs . reserve (8 * 1024) ; } let mut buf = tokio_03_to_read_buf (bs) ; ready ! (read . poll_read (cx , & mut buf)) ? ; unsafe { let n = buf . filled () . len () ; bs . advance_mut (n) ; Poll :: Ready (Ok (n)) } }
};
}
