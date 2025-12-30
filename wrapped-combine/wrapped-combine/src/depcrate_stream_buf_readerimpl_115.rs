// Generated macro for impl_115 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_115 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_115"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] impl < R > CombineRead < R , dyn tokio_03_dep :: io :: AsyncRead > for Buffer where R : tokio_03_dep :: io :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > { tokio_03_read_buf (cx , read , & mut self . 0) } }
};
}
