// Generated macro for impl_31 (impl)
macro_rules! Depcrate_futures_03impl_31 {
() => {
// Module: crate::futures_03
// Provides: {"impl_31"}
// Dependencies: {}
impl < T : futures :: io :: AsyncRead + Unpin + ? Sized > embedded_io_async :: Read for FromFutures < T > { async fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { poll_fn (| cx | Pin :: new (& mut self . inner) . poll_read (cx , buf)) . await } }
};
}
