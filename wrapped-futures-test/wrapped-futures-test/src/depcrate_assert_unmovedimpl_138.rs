// Generated macro for impl_138 (impl)
macro_rules! Depcrate_assert_unmovedimpl_138 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_138"}
// Dependencies: {}
impl < W : AsyncWrite > AsyncWrite for AssertUnmoved < W > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . poll_with (| w | w . poll_write (cx , buf)) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { self . poll_with (| w | w . poll_write_vectored (cx , bufs)) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_with (| w | w . poll_flush (cx)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_with (| w | w . poll_close (cx)) } }
};
}
