// Generated macro for virtio (module)
macro_rules! Depcrate_drivers_netvirtio {
() => {
// Module: crate::drivers::net
// Provides: {"virtio"}
// Dependencies: {}
# [cfg (all (not (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci"))) , not (feature = "rtl8139") , feature = "virtio-net" ,))] pub mod virtio ;
};
}
