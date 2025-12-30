// Generated macro for impl_112 (impl)
macro_rules! Depcrate_connection_instrumentationimpl_112 {
() => {
// Module: crate::connection::instrumentation
// Provides: {"impl_112"}
// Dependencies: {}
impl Instrumentation for Box < dyn Instrumentation > { fn on_connection_event (& mut self , event : InstrumentationEvent < '_ >) { self . deref_mut () . on_connection_event (event) } }
};
}
