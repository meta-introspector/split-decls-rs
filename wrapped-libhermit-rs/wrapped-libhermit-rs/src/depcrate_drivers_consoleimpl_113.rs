// Generated macro for impl_113 (impl)
macro_rules! Depcrate_drivers_consoleimpl_113 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_113"}
// Dependencies: {}
impl RxQueue { pub fn new () -> Self { Self { vq : None , packet_size : crate :: CONSOLE_PACKET_SIZE , } } pub fn add (& mut self , mut vq : VirtQueue) { const BUFF_PER_PACKET : u16 = 2 ; let num_packets = vq . size () / BUFF_PER_PACKET ; fill_queue (& mut vq , num_packets , self . packet_size) ; self . vq = Some (vq) ; } pub fn enable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . enable_notifs () ; } } pub fn disable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . disable_notifs () ; } } fn has_packet (& self) -> bool { self . vq . iter () . any (| vq | vq . has_used_buffers ()) } fn get_next (& mut self) -> Option < UsedBufferToken > { self . vq . as_mut () . unwrap () . try_recv () . ok () } pub fn process_packet < F > (& mut self , mut f : F) -> Result < usize , DriverError > where F : FnMut (& [u8]) -> usize , { if let Some (mut buffer_tkn) = self . get_next () { let packet = buffer_tkn . used_recv_buff . pop_front_vec () . unwrap () ; if let Some (ref mut vq) = self . vq { let result = f (& packet [..]) ; fill_queue (vq , 1 , self . packet_size) ; return Ok (result) ; } else { panic ! ("Invalid length of receive queue") ; } } Ok (0) } }
};
}
