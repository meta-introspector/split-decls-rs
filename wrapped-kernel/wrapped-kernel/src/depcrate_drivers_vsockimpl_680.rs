// Generated macro for impl_680 (impl)
macro_rules! Depcrate_drivers_vsockimpl_680 {
() => {
// Module: crate::drivers::vsock
// Provides: {"impl_680"}
// Dependencies: {}
impl RxQueue { pub fn new () -> Self { Self { vq : None , packet_size : crate :: VSOCK_PACKET_SIZE , } } pub fn add (& mut self , mut vq : VirtQueue) { const BUFF_PER_PACKET : u16 = 2 ; let num_packets = vq . size () / BUFF_PER_PACKET ; info ! ("num_packets {num_packets}") ; fill_queue (& mut vq , num_packets , self . packet_size) ; self . vq = Some (vq) ; } pub fn enable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . enable_notifs () ; } } pub fn disable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . disable_notifs () ; } } fn get_next (& mut self) -> Option < UsedBufferToken > { self . vq . as_mut () . unwrap () . try_recv () . ok () } pub fn process_packet < F > (& mut self , mut f : F) where F : FnMut (& Hdr , & [u8]) , { while let Some (mut buffer_tkn) = self . get_next () { let header = unsafe { buffer_tkn . used_recv_buff . pop_front_downcast :: < Hdr > () . unwrap () } ; let packet = buffer_tkn . used_recv_buff . pop_front_vec () . unwrap () ; if let Some (ref mut vq) = self . vq { f (& header , & packet [..]) ; fill_queue (vq , 1 , self . packet_size) ; } else { panic ! ("Invalid length of receive queue") ; } } } }
};
}
