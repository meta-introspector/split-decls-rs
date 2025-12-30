// Generated macro for impl_111 (impl)
macro_rules! Depcrate_connection_instrumentationimpl_111 {
() => {
// Module: crate::connection::instrumentation
// Provides: {"impl_111"}
// Dependencies: {}
impl < F > Instrumentation for F where F : FnMut (InstrumentationEvent < '_ >) + Send + 'static , { fn on_connection_event (& mut self , event : InstrumentationEvent < '_ >) { (self) (event) } }
};
}
