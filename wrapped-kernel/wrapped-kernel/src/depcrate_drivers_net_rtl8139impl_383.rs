// Generated macro for impl_383 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_383 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_383"}
// Dependencies: {}
impl NetworkDriver for RTL8139Driver { # [doc = " Returns the MAC address of the network interface"] fn get_mac_address (& self) -> [u8 ; 6] { self . mac } fn has_packet (& self) -> bool { let cmd = self . regs . as_ptr () . cr () . read () ; if (cmd & CR_BUFE) != CR_BUFE { let header = self . rx_fields . rx_peek_u16 () ; if header & ISR_ROK == ISR_ROK { return true ; } else { warn ! ("RTL8192: invalid header {:#x}, rx_pos {}\n" , header , self . rx_fields . rxpos) ; } } false } fn set_polling_mode (& mut self , value : bool) { if value { self . regs . as_mut_ptr () . imr () . write (le16 :: from (INT_MASK_NO_ROK)) ; } else { self . regs . as_mut_ptr () . imr () . write (le16 :: from (INT_MASK)) ; } } fn handle_interrupt (& mut self) { let isr_contents = self . regs . as_ptr () . isr () . read () . to_ne () ; if (isr_contents & ISR_TOK) == ISR_TOK { self . tx_handler () ; } if (isr_contents & ISR_RER) == ISR_RER { error ! ("RTL88139: RX error detected!\n") ; } if (isr_contents & ISR_TER) == ISR_TER { trace ! ("RTL88139r: TX error detected!\n") ; } if (isr_contents & ISR_RXOVW) == ISR_RXOVW { trace ! ("RTL88139: RX overflow detected!\n") ; } self . regs . as_mut_ptr () . isr () . write (le16 :: from (isr_contents & (ISR_RXOVW | ISR_TER | ISR_RER | ISR_TOK | ISR_ROK) ,)) ; } }
};
}
