// Generated macro for impl_231 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_231 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'a > smoltcp :: phy :: TxToken for TxToken < 'a > { fn consume < R , F > (self , len : usize , f : F) -> R where F : FnOnce (& mut [u8]) -> R , { debug ! ("get_tx_buffer") ; assert ! (len as u32 <= TX_BUF_LEN , "TX buffer is too small") ; self . tx_fields . handle_interrupt () ; let word1_addr = (self . tx_fields . txbuffer_list + u64 :: from (self . buffer_index * 8 + 4)) . as_mut_ptr :: < u32 > () ; let word1 = unsafe { core :: ptr :: read_volatile (word1_addr) } ; unsafe { core :: ptr :: write_volatile (word1_addr , word1 & (! TX_DESC_USED)) ; } let buffer = (self . tx_fields . txbuffer + u64 :: from (self . buffer_index * TX_BUF_LEN)) . as_mut_ptr :: < u8 > () ; let buffer = unsafe { slice :: from_raw_parts_mut (buffer , len) } ; let result = f (buffer) ; debug ! ("send_tx_buffer") ; let word1_addr = (self . tx_fields . txbuffer_list + u64 :: from (self . buffer_index * 8 + 4)) . as_mut_ptr :: < u32 > () ; let word1 = unsafe { core :: ptr :: read_volatile (word1_addr) } ; unsafe { core :: ptr :: write_volatile (word1_addr , (word1 & TX_DESC_WRAP) | TX_DESC_LAST | len as u32 ,) ; (* self . tx_fields . gem) . network_control . modify (NetworkControl :: TXEN :: SET) ; (* self . tx_fields . gem) . network_control . modify (NetworkControl :: STARTTX :: SET) ; } let word1_addr = (self . tx_fields . txbuffer_list + u64 :: from (self . buffer_index * 8 + 4)) . as_mut_ptr :: < u32 > () ; let word1 = unsafe { core :: ptr :: read_volatile (word1_addr) } ; unsafe { core :: ptr :: write_volatile (word1_addr , word1 | TX_DESC_USED) ; } result } }
};
}
