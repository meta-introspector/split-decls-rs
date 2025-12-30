// Generated macro for impl_263 (impl)
macro_rules! Depcrate_session_async_sessionimpl_263 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_263"}
// Dependencies: {}
impl < S : AsyncRead + Unpin > AsyncBufRead for BufferedStream < S > { fn poll_fill_buf (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { if self . buffer . is_empty () { let mut buf = [0 ; 128] ; let n = ready ! (Pin :: new (& mut self . stream) . poll_read (cx , & mut buf)) ? ; self . keep (& buf [.. n]) ; } let buf = self . get_mut () . buffer () ; Poll :: Ready (Ok (buf)) } fn consume (mut self : Pin < & mut Self > , amt : usize) { let _ = self . buffer . drain (.. amt) ; self . length -= amt ; } }
};
}
