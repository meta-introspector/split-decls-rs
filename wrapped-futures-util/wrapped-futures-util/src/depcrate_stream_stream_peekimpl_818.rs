// Generated macro for impl_818 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_818 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_818"}
// Dependencies: {}
impl < St : Stream > FusedStream for Peekable < St > { fn is_terminated (& self) -> bool { self . peeked . is_none () && self . stream . is_terminated () } }
};
}
