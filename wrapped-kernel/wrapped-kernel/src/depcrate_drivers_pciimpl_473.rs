// Generated macro for impl_473 (impl)
macro_rules! Depcrate_drivers_pciimpl_473 {
() => {
// Module: crate::drivers::pci
// Provides: {"impl_473"}
// Dependencies: {}
impl PciDriver { # [cfg (feature = "console")] fn get_console_driver (& self) -> Option < & InterruptTicketMutex < VirtioConsoleDriver > > { # [allow (unreachable_patterns)] match self { Self :: VirtioConsole (drv) => Some (drv) , _ => None , } } # [cfg (feature = "vsock")] fn get_vsock_driver (& self) -> Option < & InterruptTicketMutex < VirtioVsockDriver > > { # [allow (unreachable_patterns)] match self { Self :: VirtioVsock (drv) => Some (drv) , _ => None , } } # [cfg (feature = "fuse")] fn get_filesystem_driver (& self) -> Option < & InterruptTicketMutex < VirtioFsDriver > > { match self { Self :: VirtioFs (drv) => Some (drv) , # [allow (unreachable_patterns)] _ => None , } } fn get_interrupt_handler (& self) -> (InterruptLine , fn ()) { # [allow (unreachable_patterns)] match self { # [cfg (feature = "vsock")] Self :: VirtioVsock (drv) => { fn vsock_handler () { if let Some (driver) = get_vsock_driver () { driver . lock () . handle_interrupt () ; } } let irq_number = drv . lock () . get_interrupt_number () ; (irq_number , vsock_handler) } # [cfg (feature = "fuse")] Self :: VirtioFs (drv) => { fn fuse_handler () { } let irq_number = drv . lock () . get_interrupt_number () ; (irq_number , fuse_handler) } # [cfg (feature = "console")] Self :: VirtioConsole (drv) => { fn console_handler () { if let Some (driver) = get_console_driver () { driver . lock () . handle_interrupt () ; } } let irq_number = drv . lock () . get_interrupt_number () ; (irq_number , console_handler) } _ => todo ! () , } } }
};
}
