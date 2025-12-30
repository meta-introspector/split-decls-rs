// Generated macro for impl_232 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_232 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_232"}
// Dependencies: {}
impl Drop for TxToken < '_ > { fn drop (& mut self) { self . tx_fields . txbuffer_reserved [usize :: try_from (self . buffer_index) . unwrap ()] = false ; } }
};
}
