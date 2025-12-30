// Generated macro for init (function)
macro_rules! Depcrate_driversinit {
() => {
// Module: crate::drivers
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { # [cfg (feature = "pci")] crate :: drivers :: pci :: init () ; # [cfg (all (not (feature = "pci") , target_arch = "x86_64" , feature = "virtio-net"))] crate :: arch :: x86_64 :: kernel :: mmio :: init_drivers () ; # [cfg (all (not (feature = "pci") , target_arch = "aarch64" , any (feature = "console" , feature = "virtio-net") ,))] crate :: arch :: aarch64 :: kernel :: mmio :: init_drivers () ; # [cfg (target_arch = "riscv64")] crate :: arch :: riscv64 :: kernel :: init_drivers () ; crate :: arch :: interrupts :: install_handlers () ; }
};
}
