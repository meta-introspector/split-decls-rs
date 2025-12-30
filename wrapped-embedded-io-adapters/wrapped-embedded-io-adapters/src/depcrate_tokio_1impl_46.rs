// Generated macro for impl_46 (impl)
macro_rules! Depcrate_tokio_1impl_46 {
() => {
// Module: crate::tokio_1
// Provides: {"impl_46"}
// Dependencies: {}
impl < T : tokio :: io :: AsyncWrite + Unpin + ? Sized > embedded_io_async :: Write for FromTokio < T > { async fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { match poll_fn (| cx | Pin :: new (& mut self . inner) . poll_write (cx , buf)) . await { Ok (0) if ! buf . is_empty () => Err (std :: io :: ErrorKind :: WriteZero . into ()) , Ok (n) => Ok (n) , Err (e) => Err (e) , } } async fn flush (& mut self) -> Result < () , Self :: Error > { poll_fn (| cx | Pin :: new (& mut self . inner) . poll_flush (cx)) . await } }
};
}
