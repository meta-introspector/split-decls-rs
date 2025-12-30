// Generated macro for virtio (module)
macro_rules! Depcrate_driversvirtio {
() => {
// Module: crate::drivers
// Provides: {"virtio"}
// Dependencies: {}
# [cfg (any (all (not (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci"))) , not (feature = "rtl8139") , feature = "virtio-net" ,) , feature = "fuse" , feature = "vsock" , feature = "console" ,))] pub mod virtio ;
};
}
