// Generated macro for impl_159 (impl)
macro_rules! Depcrate_interleave_pendingimpl_159 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_159"}
// Dependencies: {}
impl < S : AsyncSeek > AsyncSeek for InterleavePending < S > { fn poll_seek (self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { self . poll_with (cx , | s , cx | s . poll_seek (cx , pos)) } }
};
}
