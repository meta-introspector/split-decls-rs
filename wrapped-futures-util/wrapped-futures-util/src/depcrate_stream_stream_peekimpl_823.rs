// Generated macro for impl_823 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_823 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_823"}
// Dependencies: {}
impl < St : Stream > FusedFuture for Peek < '_ , St > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
};
}
