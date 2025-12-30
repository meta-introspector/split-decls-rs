// Generated macro for impl_1915 (impl)
macro_rules! Depcrate_sink_map_errimpl_1915 {
() => {
// Module: crate::sink::map_err
// Provides: {"impl_1915"}
// Dependencies: {}
impl < S : FusedStream , F > FusedStream for SinkMapErr < S , F > { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
};
}
