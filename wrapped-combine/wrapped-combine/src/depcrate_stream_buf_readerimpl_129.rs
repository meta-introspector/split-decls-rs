// Generated macro for impl_129 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_129 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_129"}
// Dependencies: {}
# [cfg (feature = "futures-03")] impl < R > CombineAsyncRead < BufReader < R > > for Bufferless where R : futures_io_03 :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut BufReader < R > > ,) -> Poll < io :: Result < usize > > { let me = read . project () ; poll_extend_buf (me . buf , cx , me . inner) } fn extend_buf < 'a > (& 'a mut self , mut read : Pin < & 'a mut BufReader < R > > ,) -> ExtendBuf < 'a , Self , BufReader < R > > { let me = read . as_mut () . project () ; if ! me . buf . has_remaining_mut () { me . buf . reserve (8 * 1024) ; } let bs = me . buf . chunk_mut () ; for i in 0 .. bs . len () { bs . write_byte (i , 0) ; } ExtendBuf { buffer : self , read } } }
};
}
