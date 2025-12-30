// Generated macro for impl_117 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_117 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R > CombineRead < R , dyn tokio_dep :: io :: AsyncRead > for Buffer where R : tokio_dep :: io :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > { tokio_read_buf (read , cx , & mut self . 0) } }
};
}
