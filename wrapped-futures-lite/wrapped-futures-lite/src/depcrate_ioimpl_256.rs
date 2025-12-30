// Generated macro for impl_256 (impl)
macro_rules! Depcrate_ioimpl_256 {
() => {
// Module: crate::io
// Provides: {"impl_256"}
// Dependencies: {}
impl < T : AsyncWrite + Unpin > std :: io :: Write for BlockOn < T > { fn write (& mut self , buf : & [u8]) -> Result < usize > { future :: block_on (self . 0 . write (buf)) } fn flush (& mut self) -> Result < () > { future :: block_on (self . 0 . flush ()) } }
};
}
