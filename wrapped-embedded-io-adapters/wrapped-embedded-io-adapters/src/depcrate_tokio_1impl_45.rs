// Generated macro for impl_45 (impl)
macro_rules! Depcrate_tokio_1impl_45 {
() => {
// Module: crate::tokio_1
// Provides: {"impl_45"}
// Dependencies: {}
impl < T : tokio :: io :: AsyncBufRead + Unpin + ? Sized > embedded_io_async :: BufRead for FromTokio < T > { async fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { self . inner . fill_buf () . await } fn consume (& mut self , amt : usize) { Pin :: new (& mut self . inner) . consume (amt) ; } }
};
}
