// Generated macro for impl_128 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_128 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R > CombineRead < BufReader < R > , dyn tokio_dep :: io :: AsyncRead > for Bufferless where R : tokio_dep :: io :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut BufReader < R > > ,) -> Poll < io :: Result < usize > > { let me = read . project () ; tokio_read_buf (me . inner , cx , me . buf) } }
};
}
