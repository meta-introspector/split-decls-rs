// Generated macro for impl_113 (impl)
macro_rules! Depcrate_connection_instrumentationimpl_113 {
() => {
// Module: crate::connection::instrumentation
// Provides: {"impl_113"}
// Dependencies: {}
impl < T > Instrumentation for Option < T > where T : Instrumentation , { fn on_connection_event (& mut self , event : InstrumentationEvent < '_ >) { if let Some (i) = self { i . on_connection_event (event) } } }
};
}
