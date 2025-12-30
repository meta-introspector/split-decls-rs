// Generated macro for impl_240 (impl)
macro_rules! Depcrate_ioimpl_240 {
() => {
// Module: crate::io
// Provides: {"impl_240"}
// Dependencies: {}
impl < T : std :: io :: Read > AsyncRead for AssertAsync < T > { # [inline] fn poll_read (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . read (buf)) } # [inline] fn poll_read_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . read_vectored (bufs)) } }
};
}
