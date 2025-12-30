// Generated macro for get_vsock_driver (function)
macro_rules! Depcrate_drivers_pciget_vsock_driver {
() => {
// Module: crate::drivers::pci
// Provides: {"get_vsock_driver"}
// Dependencies: {}
# [cfg (feature = "vsock")] pub (crate) fn get_vsock_driver () -> Option < & 'static InterruptTicketMutex < VirtioVsockDriver > > { PCI_DRIVERS . get () ? . iter () . find_map (| drv | drv . get_vsock_driver ()) }
};
}
