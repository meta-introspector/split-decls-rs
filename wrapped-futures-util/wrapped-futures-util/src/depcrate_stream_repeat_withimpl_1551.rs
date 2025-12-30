// Generated macro for impl_1551 (impl)
macro_rules! Depcrate_stream_repeat_withimpl_1551 {
() => {
// Module: crate::stream::repeat_with
// Provides: {"impl_1551"}
// Dependencies: {}
impl < A , F : FnMut () -> A > FusedStream for RepeatWith < F > { fn is_terminated (& self) -> bool { false } }
};
}
