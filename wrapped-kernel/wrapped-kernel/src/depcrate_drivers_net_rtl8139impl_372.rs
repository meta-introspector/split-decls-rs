// Generated macro for impl_372 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_372 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_372"}
// Dependencies: {}
impl RxFields { fn rx_peek_u16 (& self) -> u16 { u16 :: from_le_bytes (self . rxbuffer [self . rxpos ..] [.. mem :: size_of :: < u16 > ()] . try_into () . unwrap () ,) } fn advance_rxpos (& mut self , count : usize) { self . rxpos += count ; self . rxpos %= RX_BUF_LEN ; } }
};
}
