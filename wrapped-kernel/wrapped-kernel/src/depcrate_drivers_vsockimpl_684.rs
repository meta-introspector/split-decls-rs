// Generated macro for impl_684 (impl)
macro_rules! Depcrate_drivers_vsockimpl_684 {
() => {
// Module: crate::drivers::vsock
// Provides: {"impl_684"}
// Dependencies: {}
impl EventQueue { pub fn new () -> Self { Self { vq : None , packet_size : 128u32 , } } # [doc = " Adds a given queue to the underlying vector and populates the queue with RecvBuffers."] # [doc = ""] # [doc = " Queues are all populated according to Virtio specification v1.1. - 5.1.6.3.1"] fn add (& mut self , mut vq : VirtQueue) { const BUFF_PER_PACKET : u16 = 2 ; let num_packets = vq . size () / BUFF_PER_PACKET ; fill_queue (& mut vq , num_packets , self . packet_size) ; self . vq = Some (vq) ; } pub fn enable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . enable_notifs () ; } } pub fn disable_notifs (& mut self) { if let Some (ref mut vq) = self . vq { vq . disable_notifs () ; } } }
};
}
