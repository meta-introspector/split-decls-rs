// Generated macro for impl_127 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_127 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] impl < R > CombineRead < BufReader < R > , dyn tokio_03_dep :: io :: AsyncRead > for Bufferless where R : tokio_03_dep :: io :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut BufReader < R > > ,) -> Poll < io :: Result < usize > > { let me = read . project () ; tokio_03_read_buf (cx , me . inner , me . buf) } }
};
}
