// Generated macro for get_filesystem_driver (function)
macro_rules! Depcrate_drivers_pciget_filesystem_driver {
() => {
// Module: crate::drivers::pci
// Provides: {"get_filesystem_driver"}
// Dependencies: {}
# [cfg (feature = "fuse")] pub (crate) fn get_filesystem_driver () -> Option < & 'static InterruptTicketMutex < VirtioFsDriver > > { PCI_DRIVERS . get () ? . iter () . find_map (| drv | drv . get_filesystem_driver ()) }
};
}
