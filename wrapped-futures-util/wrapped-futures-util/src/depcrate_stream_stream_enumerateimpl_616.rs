// Generated macro for impl_616 (impl)
macro_rules! Depcrate_stream_stream_enumerateimpl_616 {
() => {
// Module: crate::stream::stream::enumerate
// Provides: {"impl_616"}
// Dependencies: {}
impl < St : Stream + FusedStream > FusedStream for Enumerate < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
