// Generated macro for VirtioFsDriver (struct)
macro_rules! Depcrate_drivers_fs_virtio_fsVirtioFsDriver {
() => {
// Module: crate::drivers::fs::virtio_fs
// Provides: {"VirtioFsDriver"}
// Dependencies: {}
# [doc = " Virtio file system driver struct."] # [doc = ""] # [doc = " Struct allows to control devices virtqueues as also"] # [doc = " the device itself."] # [allow (dead_code)] pub (crate) struct VirtioFsDriver { pub (super) dev_cfg : FsDevCfg , pub (super) com_cfg : ComCfg , pub (super) isr_stat : IsrStatus , pub (super) notif_cfg : NotifCfg , pub (super) vqueues : Vec < VirtQueue > , pub (super) irq : InterruptLine , }
};
}
