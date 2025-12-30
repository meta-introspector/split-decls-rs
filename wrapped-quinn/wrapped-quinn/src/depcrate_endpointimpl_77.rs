// Generated macro for impl_77 (impl)
macro_rules! Depcrate_endpointimpl_77 {
() => {
// Module: crate::endpoint
// Provides: {"impl_77"}
// Dependencies: {}
impl Drop for State { fn drop (& mut self) { for incoming in self . recv_state . incoming . drain (..) { self . inner . ignore (incoming) ; } } }
};
}
