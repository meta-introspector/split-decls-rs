// Generated macro for impl_140 (impl)
macro_rules! Depcrate_assert_unmovedimpl_140 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_140"}
// Dependencies: {}
impl < R : AsyncBufRead > AsyncBufRead for AssertUnmoved < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { self . poll_with (| r | r . poll_fill_buf (cx)) } fn consume (self : Pin < & mut Self > , amt : usize) { self . poll_with (| r | r . consume (amt)) } }
};
}
