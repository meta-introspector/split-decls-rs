// Generated macro for impl_90 (impl)
macro_rules! Depcrate_collectorimpl_90 {
() => {
// Module: crate::collector
// Provides: {"impl_90"}
// Dependencies: {}
impl Collector { # [doc = " Creates a new collector."] pub fn new () -> Self { Self :: default () } # [doc = " Registers a new handle for the collector."] pub fn register (& self) -> LocalHandle { Local :: register (self) } }
};
}
