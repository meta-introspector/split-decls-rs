// Generated macro for impl_421 (impl)
macro_rules! Depcrate_drivers_net_virtioimpl_421 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"impl_421"}
// Dependencies: {}
impl TxQueues { pub fn new (vqs : Vec < VirtQueue > , dev_cfg : & NetDevCfg) -> Self { Self { vqs , buf_size : determine_mtu (dev_cfg) . into () , } } # [allow (dead_code)] fn enable_notifs (& mut self) { for vq in & mut self . vqs { vq . enable_notifs () ; } } # [allow (dead_code)] fn disable_notifs (& mut self) { for vq in & mut self . vqs { vq . disable_notifs () ; } } # [doc = " Polls all queues for buffers whose transmission has been completed and returns the number of such buffers."] fn poll (& mut self) -> u32 { let mut released_buffers = 0u32 ; for vq in & mut self . vqs { while vq . try_recv () . is_ok () { released_buffers += 1 ; } } released_buffers } fn add (& mut self , vq : VirtQueue) { self . vqs . push (vq) ; } }
};
}
