// Generated macro for impl_230 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_230 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_230"}
// Dependencies: {}
impl Drop for RxToken < '_ > { fn drop (& mut self) { self . rx_fields . rxbuffer_reserved [usize :: try_from (self . buffer_index) . unwrap ()] = false ; } }
};
}
