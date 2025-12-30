// Generated macro for impl_429 (impl)
macro_rules! Depcrate_drivers_net_virtioimpl_429 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"impl_429"}
// Dependencies: {}
impl smoltcp :: phy :: RxToken for RxToken < '_ > { fn consume < R , F > (self , f : F) -> R where F : FnOnce (& [u8]) -> R , { let Some (mut buffer_tkn) = self . recv_vqs . get_next () else { return f (& []) ; } ; let first_header = unsafe { buffer_tkn . used_recv_buff . pop_front_downcast :: < Hdr > () . unwrap () } ; let first_packet = buffer_tkn . used_recv_buff . pop_front_vec () . unwrap () ; let num_buffers = if self . is_mrg_rxbuf_enabled { first_header . num_buffers . to_ne () } else { 1 } ; let mut combined_packets = first_packet ; let first_tkn = buffer_token_from_hdr (unsafe { transmute :: < Box < Hdr , DeviceAlloc > , Box < MaybeUninit < Hdr > , DeviceAlloc > > (first_header) } , self . recv_vqs . buf_size ,) ; self . recv_vqs . vqs [0] . dispatch (first_tkn , false , BufferType :: Direct) . unwrap () ; for _ in 1 .. num_buffers { let mut buffer_tkn = self . recv_vqs . get_next () . unwrap () ; let (header_descriptor , used_len) = buffer_tkn . used_recv_buff . pop_front_raw () . unwrap () ; combined_packets . extend_from_slice (unsafe { core :: slice :: from_raw_parts ((& raw const * header_descriptor) . cast :: < u8 > () , used_len) }) ; let packet = buffer_tkn . used_recv_buff . pop_front_vec () . unwrap () ; combined_packets . extend_from_slice (& packet) ; let header = header_descriptor . downcast :: < MaybeUninit < Hdr > > () . unwrap () ; let tkn = buffer_token_from_hdr (header , self . recv_vqs . buf_size ,) ; self . recv_vqs . vqs [0] . dispatch (tkn , false , BufferType :: Direct) . unwrap () ; } f (& combined_packets) } }
};
}
