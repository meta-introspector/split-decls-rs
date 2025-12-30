// Generated macro for impl_44 (impl)
macro_rules! Depcrate_connectionimpl_44 {
() => {
// Module: crate::connection
// Provides: {"impl_44"}
// Dependencies: {}
impl Drop for State { fn drop (& mut self) { if ! self . inner . is_drained () { let _ = self . endpoint_events . send ((self . handle , proto :: EndpointEvent :: drained ())) ; } } }
};
}
