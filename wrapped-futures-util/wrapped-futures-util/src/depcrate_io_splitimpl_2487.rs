// Generated macro for impl_2487 (impl)
macro_rules! Depcrate_io_splitimpl_2487 {
() => {
// Module: crate::io::split
// Provides: {"impl_2487"}
// Dependencies: {}
impl < W : AsyncWrite > AsyncWrite for WriteHalf < W > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_write (cx , buf)) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_write_vectored (cx , bufs)) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_flush (cx)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_close (cx)) } }
};
}
