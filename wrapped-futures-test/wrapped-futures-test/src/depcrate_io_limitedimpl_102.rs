// Generated macro for impl_102 (impl)
macro_rules! Depcrate_io_limitedimpl_102 {
() => {
// Module: crate::io::limited
// Provides: {"impl_102"}
// Dependencies: {}
impl < R : AsyncRead > AsyncRead for Limited < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { let this = self . project () ; let limit = cmp :: min (* this . limit , buf . len ()) ; this . io . poll_read (cx , & mut buf [.. limit]) } }
};
}
