// Generated macro for impl_112 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_112 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (feature = "futures-03")] impl < R > CombineAsyncRead < R > for Buffer where R : futures_io_03 :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > { poll_extend_buf (& mut self . 0 , cx , read) } fn extend_buf < 'a > (& 'a mut self , read : Pin < & 'a mut R >) -> ExtendBuf < 'a , Self , R > { if ! self . 0 . has_remaining_mut () { self . 0 . reserve (8 * 1024) ; } let bs = self . 0 . chunk_mut () ; for i in 0 .. bs . len () { bs . write_byte (i , 0) ; } ExtendBuf { buffer : self , read } } }
};
}
