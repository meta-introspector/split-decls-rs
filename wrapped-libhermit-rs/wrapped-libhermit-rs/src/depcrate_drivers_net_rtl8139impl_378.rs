// Generated macro for impl_378 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_378 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_378"}
// Dependencies: {}
impl < 'a > smoltcp :: phy :: RxToken for RxToken < 'a > { fn consume < R , F > (mut self , f : F) -> R where F : FnOnce (& [u8]) -> R , { self . rx_fields . advance_rxpos (mem :: size_of :: < u16 > ()) ; let length = self . rx_fields . rx_peek_u16 () - 4 ; let pos = (self . rx_fields . rxpos + mem :: size_of :: < u16 > ()) % RX_BUF_LEN ; let mut vec_data = Vec :: with_capacity (length as usize) ; let frame = if pos + length as usize > RX_BUF_LEN { let first = & self . rx_fields . rxbuffer [pos .. RX_BUF_LEN] ; let second = & self . rx_fields . rxbuffer [.. length as usize - first . len ()] ; vec_data . extend_from_slice (first) ; vec_data . extend_from_slice (second) ; vec_data . as_slice () } else { & self . rx_fields . rxbuffer [pos ..] [.. length . into ()] } ; let result = f (frame) ; self . consume_current_buffer () ; result } }
};
}
