// Generated macro for get_console_driver (function)
macro_rules! Depcrate_drivers_pciget_console_driver {
() => {
// Module: crate::drivers::pci
// Provides: {"get_console_driver"}
// Dependencies: {}
# [cfg (feature = "console")] pub (crate) fn get_console_driver () -> Option < & 'static InterruptTicketMutex < VirtioConsoleDriver > > { PCI_DRIVERS . get () ? . iter () . find_map (| drv | drv . get_console_driver ()) }
};
}
