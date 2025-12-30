// Generated macro for impl_122 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_122 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_122"}
// Dependencies: {}
impl < R > CombineSyncRead < BufReader < R > > for Bufferless where R : Read , { fn extend_buf_sync (& mut self , read : & mut BufReader < R >) -> io :: Result < usize > { extend_buf_sync (& mut read . buf , & mut read . inner) } }
};
}
