// Generated macro for impl_34 (impl)
macro_rules! Depcrate_futures_03impl_34 {
() => {
// Module: crate::futures_03
// Provides: {"impl_34"}
// Dependencies: {}
impl < T : futures :: io :: AsyncSeek + Unpin + ? Sized > embedded_io_async :: Seek for FromFutures < T > { async fn seek (& mut self , pos : embedded_io :: SeekFrom) -> Result < u64 , Self :: Error > { poll_fn (move | cx | Pin :: new (& mut self . inner) . poll_seek (cx , pos . into ())) . await } }
};
}
