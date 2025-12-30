// Generated macro for impl_605 (impl)
macro_rules! Depcrate_stream_stream_cycleimpl_605 {
() => {
// Module: crate::stream::stream::cycle
// Provides: {"impl_605"}
// Dependencies: {}
impl < St > FusedStream for Cycle < St > where St : Clone + Stream , { fn is_terminated (& self) -> bool { matches ! (self . size_hint () , (0 , Some (0))) } }
};
}
