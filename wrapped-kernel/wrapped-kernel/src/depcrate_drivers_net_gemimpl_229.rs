// Generated macro for impl_229 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_229 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'a > smoltcp :: phy :: RxToken for RxToken < 'a > { fn consume < R , F > (mut self , f : F) -> R where F : FnOnce (& [u8]) -> R , { debug ! ("receive_rx_buffer") ; let word1_addr = self . rx_fields . rxbuffer_list + u64 :: from (self . buffer_index * 8 + 4) ; let word1_entry = unsafe { core :: ptr :: read_volatile (word1_addr . as_mut_ptr :: < u32 > ()) } ; let length = word1_entry & 0x1fff ; debug ! ("Received frame in buffer {}, length: {length}" , self . buffer_index) ; let buffer = unsafe { core :: slice :: from_raw_parts_mut ((self . rx_fields . rxbuffer . as_usize () + (self . buffer_index * RX_BUF_LEN) as usize) as * mut u8 , length as usize ,) } ; trace ! ("BUFFER: {buffer:x?}") ; let res = f (buffer) ; self . rx_buffer_consumed () ; res } }
};
}
