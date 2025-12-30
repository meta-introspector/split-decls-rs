// Generated macro for tokio_read_buf (function)
macro_rules! Depcrate_stream_buf_readertokio_read_buf {
() => {
// Module: crate::stream::buf_reader
// Provides: {"tokio_read_buf"}
// Dependencies: {}
# [cfg (feature = "tokio")] fn tokio_read_buf (read : Pin < & mut impl tokio_dep :: io :: AsyncRead > , cx : & mut Context < '_ > , bs : & mut bytes :: BytesMut ,) -> Poll < io :: Result < usize > > { if ! bs . has_remaining_mut () { bs . reserve (8 * 1024) ; } tokio_util :: io :: poll_read_buf (read , cx , bs) }
};
}
