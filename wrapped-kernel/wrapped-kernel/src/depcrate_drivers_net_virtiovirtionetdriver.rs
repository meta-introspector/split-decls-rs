// Generated macro for VirtioNetDriver (struct)
macro_rules! Depcrate_drivers_net_virtioVirtioNetDriver {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"VirtioNetDriver"}
// Dependencies: {}
# [doc = " Virtio network driver struct."] # [doc = ""] # [doc = " Struct allows to control devices virtqueues as also"] # [doc = " the device itself."] pub (crate) struct VirtioNetDriver < T = Init > { pub (super) dev_cfg : NetDevCfg , pub (super) com_cfg : ComCfg , pub (super) isr_stat : IsrStatus , pub (super) notif_cfg : NotifCfg , pub (super) inner : T , pub (super) num_vqs : u16 , pub (super) irq : InterruptLine , pub (super) checksums : ChecksumCapabilities , }
};
}
