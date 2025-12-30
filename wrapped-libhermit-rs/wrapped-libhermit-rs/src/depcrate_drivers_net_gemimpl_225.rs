// Generated macro for impl_225 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_225 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_225"}
// Dependencies: {}
impl GEMDriver { # [doc = " Returns the index of the next received frame"] fn next_rx_index (& self) -> Option < u32 > { for i in 0 .. RX_BUF_NUM { let index = (i + self . rx_counter) % RX_BUF_NUM ; let word0_addr = (self . rx_fields . rxbuffer_list + u64 :: from (index * 8)) ; let word0_entry = unsafe { core :: ptr :: read_volatile (word0_addr . as_mut_ptr :: < u32 > ()) } ; if (word0_entry & 0x1) != 0 { return Some (index) ; } } None } # [expect (clippy :: modulo_one)] fn next_tx_index (& self) -> Option < u32 > { for i in 0 .. TX_BUF_NUM { let index = (i + self . tx_counter) % TX_BUF_NUM ; let word1_addr = (self . tx_fields . txbuffer_list + u64 :: from (index * 8 + 4)) . as_mut_ptr :: < u32 > () ; let word1 = unsafe { core :: ptr :: read_volatile (word1_addr) } ; if word1 & TX_DESC_USED != 0 && ! self . tx_fields . txbuffer_reserved [usize :: try_from (i) . unwrap ()] { return Some (i) ; } } None } # [expect (clippy :: modulo_one)] fn reserve_tx_index (& mut self , index : u32) { self . tx_counter = (index + 1) % TX_BUF_NUM ; self . tx_fields . txbuffer_reserved [usize :: try_from (index) . unwrap ()] = true ; } }
};
}
