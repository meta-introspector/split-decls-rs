// Generated macro for impl_139 (impl)
macro_rules! Depcrate_assert_unmovedimpl_139 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_139"}
// Dependencies: {}
impl < S : AsyncSeek > AsyncSeek for AssertUnmoved < S > { fn poll_seek (self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { self . poll_with (| s | s . poll_seek (cx , pos)) } }
};
}
