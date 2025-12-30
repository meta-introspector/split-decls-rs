// Generated macro for impl_44 (impl)
macro_rules! Depcrate_tokio_1impl_44 {
() => {
// Module: crate::tokio_1
// Provides: {"impl_44"}
// Dependencies: {}
impl < T : tokio :: io :: AsyncRead + Unpin + ? Sized > embedded_io_async :: Read for FromTokio < T > { async fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { if buf . is_empty () { return Ok (0) ; } poll_fn (| cx | { let mut buf = tokio :: io :: ReadBuf :: new (buf) ; match Pin :: new (& mut self . inner) . poll_read (cx , & mut buf) { Poll :: Ready (r) => match r { Ok (()) => Poll :: Ready (Ok (buf . filled () . len ())) , Err (e) => Poll :: Ready (Err (e)) , } , Poll :: Pending => Poll :: Pending , } }) . await } }
};
}
