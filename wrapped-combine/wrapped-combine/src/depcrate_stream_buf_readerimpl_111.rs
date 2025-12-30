// Generated macro for impl_111 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_111 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_111"}
// Dependencies: {}
impl < R > CombineSyncRead < R > for Buffer where R : Read , { fn extend_buf_sync (& mut self , read : & mut R) -> io :: Result < usize > { extend_buf_sync (& mut self . 0 , read) } }
};
}
