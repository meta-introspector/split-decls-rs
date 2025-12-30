// Generated macro for impl_113 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_113 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (feature = "tokio-02")] impl < R > CombineRead < R , dyn tokio_02_dep :: io :: AsyncRead > for Buffer where R : tokio_02_dep :: io :: AsyncRead , { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > { if ! self . 0 . has_remaining_mut () { self . 0 . reserve (8 * 1024) ; } read . poll_read_buf (cx , & mut Bytes05 (& mut self . 0)) } }
};
}
