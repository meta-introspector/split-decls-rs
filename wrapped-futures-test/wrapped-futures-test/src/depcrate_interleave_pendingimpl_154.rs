// Generated macro for impl_154 (impl)
macro_rules! Depcrate_interleave_pendingimpl_154 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_154"}
// Dependencies: {}
impl < St : Stream > Stream for InterleavePending < St > { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_with (cx , St :: poll_next) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
