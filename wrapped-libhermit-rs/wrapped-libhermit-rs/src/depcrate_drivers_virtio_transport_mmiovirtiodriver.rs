// Generated macro for VirtioDriver (enum)
macro_rules! Depcrate_drivers_virtio_transport_mmioVirtioDriver {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"VirtioDriver"}
// Dependencies: {}
pub (crate) enum VirtioDriver { # [cfg (feature = "virtio-net")] Network (VirtioNetDriver) , # [cfg (feature = "console")] Console (Box < VirtioConsoleDriver >) , }
};
}
