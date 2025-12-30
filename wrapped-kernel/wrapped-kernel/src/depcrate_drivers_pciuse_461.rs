// Generated macro for use_461 (use)
macro_rules! Depcrate_drivers_pciuse_461 {
() => {
// Module: crate::drivers::pci
// Provides: {"use_461"}
// Dependencies: {}
# [cfg (any (all (feature = "virtio-net" , not (feature = "rtl8139") ,) , feature = "fuse" , feature = "vsock" , feature = "console" ,))] use crate :: drivers :: virtio :: transport :: pci :: VirtioDriver ;
};
}
