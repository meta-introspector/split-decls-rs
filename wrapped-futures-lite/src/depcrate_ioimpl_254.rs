// Generated macro for impl_254 (impl)
macro_rules! Depcrate_ioimpl_254 {
() => {
// Module: crate::io
// Provides: {"impl_254"}
// Dependencies: {}
impl < T : AsyncRead + Unpin > std :: io :: Read for BlockOn < T > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { future :: block_on (self . 0 . read (buf)) } }
};
}
