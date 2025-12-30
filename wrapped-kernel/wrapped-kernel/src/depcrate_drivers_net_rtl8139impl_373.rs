// Generated macro for impl_373 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_373 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_373"}
// Dependencies: {}
impl RxToken < '_ > { fn consume_current_buffer (& mut self) { let length = self . rx_fields . rx_peek_u16 () ; self . rx_fields . advance_rxpos (usize :: from (length) + mem :: size_of :: < u16 > ()) ; self . rx_fields . rxpos = ((self . rx_fields . rxpos + 3) & ! 0x3) % RX_BUF_LEN ; let capr : u16 = if self . rx_fields . rxpos >= 0x10 { (self . rx_fields . rxpos - 0x10) . try_into () . unwrap () } else { (RX_BUF_LEN - (0x10 - self . rx_fields . rxpos)) . try_into () . unwrap () } ; self . capr . write (le16 :: from (capr)) ; } }
};
}
