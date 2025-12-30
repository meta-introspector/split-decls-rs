// Generated macro for impl_827 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_827 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_827"}
// Dependencies: {}
impl < St : Stream > FusedFuture for PeekMut < '_ , St > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
};
}
