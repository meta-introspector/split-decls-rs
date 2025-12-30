// Generated macro for Init (struct)
macro_rules! Depcrate_drivers_net_virtioInit {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"Init"}
// Dependencies: {}
pub (crate) struct Init { pub (super) mtu : u16 , pub (super) ctrl_vq : Option < VirtQueue > , pub (super) recv_vqs : RxQueues , pub (super) send_vqs : TxQueues , # [doc = " Capacity in number of buffer descriptors, not frames."] pub (super) send_capacity : u32 , }
};
}
