// Generated macro for impl_126 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_126 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (feature = "tokio-02")] impl < R > CombineRead < BufReader < R > , dyn tokio_02_dep :: io :: AsyncRead > for Bufferless where R : tokio_02_dep :: io :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut BufReader < R > > ,) -> Poll < io :: Result < usize > > { let me = read . project () ; if ! me . buf . has_remaining_mut () { me . buf . reserve (8 * 1024) ; } tokio_02_dep :: io :: AsyncRead :: poll_read_buf (me . inner , cx , & mut Bytes05 (me . buf)) } }
};
}
