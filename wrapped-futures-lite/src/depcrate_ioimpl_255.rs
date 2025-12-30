// Generated macro for impl_255 (impl)
macro_rules! Depcrate_ioimpl_255 {
() => {
// Module: crate::io
// Provides: {"impl_255"}
// Dependencies: {}
impl < T : AsyncBufRead + Unpin > std :: io :: BufRead for BlockOn < T > { fn fill_buf (& mut self) -> Result < & [u8] > { future :: block_on (self . 0 . fill_buf ()) } fn consume (& mut self , amt : usize) { Pin :: new (& mut self . 0) . consume (amt) } }
};
}
