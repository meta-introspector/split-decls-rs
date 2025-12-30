// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < T > futures_core :: stream :: FusedStream for Receiver < T > { fn is_terminated (& self) -> bool { self . channel . queue . is_closed () && self . channel . queue . is_empty () } }
};
}
