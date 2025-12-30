// Generated macro for impl_257 (impl)
macro_rules! Depcrate_ioimpl_257 {
() => {
// Module: crate::io
// Provides: {"impl_257"}
// Dependencies: {}
impl < T : AsyncSeek + Unpin > std :: io :: Seek for BlockOn < T > { fn seek (& mut self , pos : SeekFrom) -> Result < u64 > { future :: block_on (self . 0 . seek (pos)) } }
};
}
