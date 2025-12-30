// Generated macro for impl_241 (impl)
macro_rules! Depcrate_ioimpl_241 {
() => {
// Module: crate::io
// Provides: {"impl_241"}
// Dependencies: {}
impl < T : std :: io :: Write > AsyncWrite for AssertAsync < T > { # [inline] fn poll_write (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . write (buf)) } # [inline] fn poll_write_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . write_vectored (bufs)) } # [inline] fn poll_flush (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () > > { assert_async_wrapio (move | | self . 0 . flush ()) } # [inline] fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { self . poll_flush (cx) } }
};
}
