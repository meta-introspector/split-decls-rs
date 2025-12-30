// Generated macro for impl_221 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_221 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_221"}
// Dependencies: {}
impl NetworkDriver for GEMDriver { # [doc = " Returns the MAC address of the network interface"] fn get_mac_address (& self) -> [u8 ; 6] { self . mac } fn has_packet (& self) -> bool { debug ! ("has_packet") ; self . next_rx_index () . is_some () } fn set_polling_mode (& mut self , value : bool) { debug ! ("set_polling_mode") ; if value { unsafe { (* self . tx_fields . gem) . int_disable . set (0x7ff_feff) ; } } else { unsafe { (* self . tx_fields . gem) . int_enable . write (Interrupts :: FRAMERX :: SET) ; } } } fn handle_interrupt (& mut self) { self . tx_fields . handle_interrupt () ; } }
};
}
