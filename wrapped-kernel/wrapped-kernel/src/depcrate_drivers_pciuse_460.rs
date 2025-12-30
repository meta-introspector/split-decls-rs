// Generated macro for use_460 (use)
macro_rules! Depcrate_drivers_pciuse_460 {
() => {
// Module: crate::drivers::pci
// Provides: {"use_460"}
// Dependencies: {}
# [cfg (any (all (feature = "virtio-net" , not (feature = "rtl8139") ,) , feature = "fuse" , feature = "vsock" , feature = "console" ,))] use crate :: drivers :: virtio :: transport :: pci as pci_virtio ;
};
}
