// Generated macro for impl_417 (impl)
macro_rules! Depcrate_drivers_net_virtioimpl_417 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"impl_417"}
// Dependencies: {}
impl RxQueues { pub fn new (vqs : Vec < VirtQueue > , dev_cfg : & NetDevCfg) -> Self { Self { vqs , buf_size : determine_rx_buf_size (dev_cfg) , } } # [doc = " Adds a given queue to the underlying vector and populates the queue with RecvBuffers."] # [doc = ""] # [doc = " Queues are all populated according to Virtio specification v1.1. - 5.1.6.3.1"] fn add (& mut self , mut vq : VirtQueue) { let num_bufs = vq . size () / constants :: BUFF_PER_PACKET ; fill_queue (& mut vq , num_bufs , self . buf_size) ; self . vqs . push (vq) ; } fn get_next (& mut self) -> Option < UsedBufferToken > { self . vqs [0] . try_recv () . ok () } fn enable_notifs (& mut self) { for vq in & mut self . vqs { vq . enable_notifs () ; } } fn disable_notifs (& mut self) { for vq in & mut self . vqs { vq . disable_notifs () ; } } fn has_packet (& self) -> bool { self . vqs . iter () . any (| vq | vq . has_used_buffers ()) } }
};
}
