// Generated macro for impl_32 (impl)
macro_rules! Depcrate_futures_03impl_32 {
() => {
// Module: crate::futures_03
// Provides: {"impl_32"}
// Dependencies: {}
impl < T : futures :: io :: AsyncBufRead + Unpin + ? Sized > embedded_io_async :: BufRead for FromFutures < T > { async fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { self . inner . fill_buf () . await } fn consume (& mut self , amt : usize) { Pin :: new (& mut self . inner) . consume (amt) ; } }
};
}
