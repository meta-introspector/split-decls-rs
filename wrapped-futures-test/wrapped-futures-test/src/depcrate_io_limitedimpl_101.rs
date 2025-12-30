// Generated macro for impl_101 (impl)
macro_rules! Depcrate_io_limitedimpl_101 {
() => {
// Module: crate::io::limited
// Provides: {"impl_101"}
// Dependencies: {}
impl < W : AsyncWrite > AsyncWrite for Limited < W > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { let this = self . project () ; this . io . poll_write (cx , & buf [.. cmp :: min (* this . limit , buf . len ())]) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . project () . io . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . project () . io . poll_close (cx) } }
};
}
