// Generated macro for impl_47 (impl)
macro_rules! Depcrate_tokio_1impl_47 {
() => {
// Module: crate::tokio_1
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : tokio :: io :: AsyncSeek + Unpin + ? Sized > embedded_io_async :: Seek for FromTokio < T > { async fn seek (& mut self , pos : embedded_io :: SeekFrom) -> Result < u64 , Self :: Error > { poll_fn (| cx | Pin :: new (& mut self . inner) . poll_complete (cx)) . await ? ; Pin :: new (& mut self . inner) . start_seek (pos . into ()) ? ; poll_fn (| cx | Pin :: new (& mut self . inner) . poll_complete (cx)) . await } }
};
}
