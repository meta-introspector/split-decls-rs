// Generated macro for impl_380 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_380 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_380"}
// Dependencies: {}
impl Drop for TxToken < '_ > { fn drop (& mut self) { self . tx_fields . remaining_bufs += 1 ; } }
};
}
