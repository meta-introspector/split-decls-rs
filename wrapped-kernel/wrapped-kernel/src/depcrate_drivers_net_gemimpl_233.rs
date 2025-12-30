// Generated macro for impl_233 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_233 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_233"}
// Dependencies: {}
impl Drop for GEMDriver { fn drop (& mut self) { debug ! ("Dropping GEMDriver!") ; unsafe { (* self . tx_fields . gem) . network_control . set (0x0) ; deallocate (self . rx_fields . rxbuffer , (RX_BUF_LEN * RX_BUF_NUM) as usize) ; deallocate (self . tx_fields . txbuffer , (TX_BUF_LEN * TX_BUF_NUM) as usize) ; deallocate (self . rx_fields . rxbuffer_list , (8 * RX_BUF_NUM) as usize) ; deallocate (self . tx_fields . txbuffer_list , (8 * TX_BUF_NUM) as usize) ; } } }
};
}
