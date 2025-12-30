// Generated macro for impl_137 (impl)
macro_rules! Depcrate_assert_unmovedimpl_137 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_137"}
// Dependencies: {}
impl < R : AsyncRead > AsyncRead for AssertUnmoved < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { self . poll_with (| r | r . poll_read (cx , buf)) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { self . poll_with (| r | r . poll_read_vectored (cx , bufs)) } }
};
}
