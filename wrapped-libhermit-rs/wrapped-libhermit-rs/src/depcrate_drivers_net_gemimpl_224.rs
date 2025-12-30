// Generated macro for impl_224 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_224 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_224"}
// Dependencies: {}
impl RxToken < '_ > { fn rx_buffer_consumed (& mut self) { debug ! ("rx_buffer_consumed: handle: {}" , self . buffer_index) ; let word0_addr = (self . rx_fields . rxbuffer_list + u64 :: from (self . buffer_index * 8)) ; let word1_addr = word0_addr + 4u64 ; unsafe { core :: ptr :: write_volatile (word1_addr . as_mut_ptr :: < u32 > () , 0) ; let word0_entry = core :: ptr :: read_volatile (word0_addr . as_mut_ptr :: < u32 > ()) ; core :: ptr :: write_volatile (word0_addr . as_mut_ptr :: < u32 > () , word0_entry & 0xffff_fffe) ; } } }
};
}
