// Generated macro for impl_33 (impl)
macro_rules! Depcrate_futures_03impl_33 {
() => {
// Module: crate::futures_03
// Provides: {"impl_33"}
// Dependencies: {}
impl < T : futures :: io :: AsyncWrite + Unpin + ? Sized > embedded_io_async :: Write for FromFutures < T > { async fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { match poll_fn (| cx | Pin :: new (& mut self . inner) . poll_write (cx , buf)) . await { Ok (0) if ! buf . is_empty () => Err (std :: io :: ErrorKind :: WriteZero . into ()) , Ok (n) => Ok (n) , Err (e) => Err (e) , } } async fn flush (& mut self) -> Result < () , Self :: Error > { poll_fn (| cx | Pin :: new (& mut self . inner) . poll_flush (cx)) . await } }
};
}
