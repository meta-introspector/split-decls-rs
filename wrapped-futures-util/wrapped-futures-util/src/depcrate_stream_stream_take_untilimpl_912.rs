// Generated macro for impl_912 (impl)
macro_rules! Depcrate_stream_stream_take_untilimpl_912 {
() => {
// Module: crate::stream::stream::take_until
// Provides: {"impl_912"}
// Dependencies: {}
impl < St , Fut > FusedStream for TakeUntil < St , Fut > where St : Stream , Fut : Future , { fn is_terminated (& self) -> bool { self . is_stopped () } }
};
}
