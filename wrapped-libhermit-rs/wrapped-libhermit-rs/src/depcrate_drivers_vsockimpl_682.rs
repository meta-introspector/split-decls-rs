// Generated macro for impl_682 (impl)
macro_rules! Depcrate_drivers_vsockimpl_682 {
() => {
// Module: crate::drivers::vsock
// Provides: {"impl_682"}
// Dependencies: {}
impl TxQueue { pub fn new () -> Self { Self { vq : None , packet_length : crate :: VSOCK_PACKET_SIZE + mem :: size_of :: < Hdr > () as u32 , } } pub fn add (& mut self , vq : VirtQueue) { self . vq = Some (vq) ; } pub fn enable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . enable_notifs () ; } } pub fn disable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . disable_notifs () ; } } fn poll (& mut self) { if let Some (ref mut vq) = self . vq { while vq . try_recv () . is_ok () { } } } # [doc = " Provides a slice to copy the packet and transfer the packet"] # [doc = " to the send queue. The caller has to create the header"] # [doc = " for the vsock interface."] pub fn send_packet < R , F > (& mut self , len : usize , f : F) -> R where F : FnOnce (& mut [u8]) -> R , { self . poll () ; if let Some (ref mut vq) = self . vq { assert ! (len < usize :: try_from (self . packet_length) . unwrap ()) ; let mut packet = Vec :: with_capacity_in (len , DeviceAlloc) ; let result = unsafe { let result = f (packet . spare_capacity_mut () . assume_init_mut ()) ; packet . set_len (len) ; result } ; let buff_tkn = AvailBufferToken :: new ({ let mut vec = SmallVec :: new () ; vec . push (BufferElem :: Vector (packet)) ; vec } , SmallVec :: new () ,) . unwrap () ; vq . dispatch (buff_tkn , false , BufferType :: Direct) . unwrap () ; result } else { panic ! ("Unable to get send queue") ; } } }
};
}
