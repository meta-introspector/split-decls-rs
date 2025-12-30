// Generated macro for loopback (module)
macro_rules! Depcrate_drivers_netloopback {
() => {
// Module: crate::drivers::net
// Provides: {"loopback"}
// Dependencies: {}
# [cfg (not (any (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci")) , feature = "rtl8139" , feature = "virtio-net" ,)))] pub mod loopback ;
};
}
