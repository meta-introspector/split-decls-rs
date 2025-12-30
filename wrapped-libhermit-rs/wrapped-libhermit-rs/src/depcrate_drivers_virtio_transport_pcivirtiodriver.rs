// Generated macro for VirtioDriver (enum)
macro_rules! Depcrate_drivers_virtio_transport_pciVirtioDriver {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"VirtioDriver"}
// Dependencies: {}
pub (crate) enum VirtioDriver { # [cfg (all (not (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci"))) , not (feature = "rtl8139") , feature = "virtio-net" ,))] Network (VirtioNetDriver) , # [cfg (feature = "console")] Console (Box < VirtioConsoleDriver >) , # [cfg (feature = "vsock")] Vsock (Box < VirtioVsockDriver >) , # [cfg (feature = "fuse")] FileSystem (VirtioFsDriver) , }
};
}
