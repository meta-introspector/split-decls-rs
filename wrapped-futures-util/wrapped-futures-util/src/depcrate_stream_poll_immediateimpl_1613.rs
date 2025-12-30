// Generated macro for impl_1613 (impl)
macro_rules! Depcrate_stream_poll_immediateimpl_1613 {
() => {
// Module: crate::stream::poll_immediate
// Provides: {"impl_1613"}
// Dependencies: {}
impl < S : Stream > super :: FusedStream for PollImmediate < S > { fn is_terminated (& self) -> bool { self . stream . is_none () } }
};
}
