// Generated macro for impl_262 (impl)
macro_rules! Depcrate_session_async_sessionimpl_262 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_262"}
// Dependencies: {}
impl < S : AsyncRead + Unpin > AsyncRead for BufferedStream < S > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { let mut rem = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let nread = std :: io :: Read :: read (& mut rem , buf) ? ; self . consume (nread) ; Poll :: Ready (Ok (nread)) } fn poll_read_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { let mut rem = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let nread = std :: io :: Read :: read_vectored (& mut rem , bufs) ? ; self . consume (nread) ; Poll :: Ready (Ok (nread)) } }
};
}
