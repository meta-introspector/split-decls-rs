// Generated macro for use_528 (use)
macro_rules! Depcrate_drivers_virtio_transport_pciuse_528 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"use_528"}
// Dependencies: {}
# [cfg (all (not (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci"))) , not (feature = "rtl8139") , feature = "virtio-net" ,))] use crate :: drivers :: net :: virtio :: VirtioNetDriver ;
};
}
