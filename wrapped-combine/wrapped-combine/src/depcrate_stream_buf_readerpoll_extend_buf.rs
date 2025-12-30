// Generated macro for poll_extend_buf (function)
macro_rules! Depcrate_stream_buf_readerpoll_extend_buf {
() => {
// Module: crate::stream::buf_reader
// Provides: {"poll_extend_buf"}
// Dependencies: {}
# [cfg (feature = "futures-03")] fn poll_extend_buf < R > (buf : & mut BytesMut , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > where R : futures_io_03 :: AsyncRead , { let n = { let bs = buf . chunk_mut () ; for i in 0 .. bs . len () { bs . write_byte (i , 0) ; } let bs = unsafe { & mut * (bs as * mut _ as * mut [u8]) } ; let n = ready ! (read . poll_read (cx , bs)) ? ; assert ! (n <= bs . len () , "AsyncRead reported that it initialized more than the number of bytes in the buffer") ; n } ; unsafe { buf . advance_mut (n) } ; Poll :: Ready (Ok (n)) }
};
}
