// Generated macro for register_driver (function)
macro_rules! Depcrate_drivers_pciregister_driver {
() => {
// Module: crate::drivers::pci
// Provides: {"register_driver"}
// Dependencies: {}
pub (crate) fn register_driver (drv : PciDriver) { PCI_DRIVERS . with (| pci_drivers | pci_drivers . unwrap () . push (drv)) ; }
};
}
