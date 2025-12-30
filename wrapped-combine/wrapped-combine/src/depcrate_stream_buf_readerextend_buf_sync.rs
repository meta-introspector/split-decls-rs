// Generated macro for extend_buf_sync (function)
macro_rules! Depcrate_stream_buf_readerextend_buf_sync {
() => {
// Module: crate::stream::buf_reader
// Provides: {"extend_buf_sync"}
// Dependencies: {}
fn extend_buf_sync < R > (buf : & mut BytesMut , read : & mut R) -> io :: Result < usize > where R : Read , { let size = 8 * 1024 ; if ! buf . has_remaining_mut () { buf . reserve (size) ; } let n = { let bs = buf . chunk_mut () ; let initial_size = bs . len () . min (size) ; let bs = & mut bs [.. initial_size] ; for i in 0 .. bs . len () { bs . write_byte (i , 0) ; } let bs = unsafe { & mut * (bs as * mut _ as * mut [u8]) } ; let n = read . read (bs) ? ; assert ! (n <= bs . len () , "AsyncRead reported that it initialized more than the number of bytes in the buffer") ; n } ; unsafe { buf . advance_mut (n) } ; Ok (n) }
};
}
