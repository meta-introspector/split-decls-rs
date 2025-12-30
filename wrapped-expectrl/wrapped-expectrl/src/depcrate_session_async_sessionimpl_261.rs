// Generated macro for impl_261 (impl)
macro_rules! Depcrate_session_async_sessionimpl_261 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_261"}
// Dependencies: {}
impl < S : AsyncRead + Unpin > BufferedStream < S > { async fn fill (& mut self) -> io :: Result < usize > { let mut buf = [0 ; 128] ; let n = self . stream . read (& mut buf) . await ? ; self . keep (& buf [.. n]) ; Ok (n) } }
};
}
