// Generated macro for impl_242 (impl)
macro_rules! Depcrate_ioimpl_242 {
() => {
// Module: crate::io
// Provides: {"impl_242"}
// Dependencies: {}
impl < T : std :: io :: Seek > AsyncSeek for AssertAsync < T > { # [inline] fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < Result < u64 > > { assert_async_wrapio (move | | self . 0 . seek (pos)) } }
};
}
