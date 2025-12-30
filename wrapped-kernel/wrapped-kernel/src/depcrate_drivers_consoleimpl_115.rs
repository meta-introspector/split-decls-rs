// Generated macro for impl_115 (impl)
macro_rules! Depcrate_drivers_consoleimpl_115 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_115"}
// Dependencies: {}
impl TxQueue { pub fn new () -> Self { Self { vq : None , packet_length : crate :: CONSOLE_PACKET_SIZE , } } pub fn add (& mut self , vq : VirtQueue) { self . vq = Some (vq) ; } pub fn enable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . enable_notifs () ; } } pub fn disable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . disable_notifs () ; } } fn poll (& mut self) { if let Some (ref mut vq) = self . vq { while vq . try_recv () . is_ok () { } } } # [doc = " Provides a slice to copy the packet and transfer the packet"] # [doc = " to the send queue. The caller has to create the header"] # [doc = " for the vsock interface."] pub fn send_packet (& mut self , buf : & [u8]) { self . poll () ; if let Some (ref mut vq) = self . vq { assert ! (buf . len () < usize :: try_from (self . packet_length) . unwrap ()) ; let mut packet = Vec :: with_capacity_in (buf . len () , DeviceAlloc) ; packet . extend_from_slice (buf) ; let buff_tkn = AvailBufferToken :: new ({ let mut vec = SmallVec :: new () ; vec . push (BufferElem :: Vector (packet)) ; vec } , SmallVec :: new () ,) . unwrap () ; vq . dispatch (buff_tkn , false , BufferType :: Direct) . unwrap () ; } else { panic ! ("Unable to get send queue") ; } } }
};
}
