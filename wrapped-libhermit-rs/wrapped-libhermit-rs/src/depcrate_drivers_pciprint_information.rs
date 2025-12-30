// Generated macro for print_information (function)
macro_rules! Depcrate_drivers_pciprint_information {
() => {
// Module: crate::drivers::pci
// Provides: {"print_information"}
// Dependencies: {}
pub (crate) fn print_information () { infoheader ! (" PCI BUS INFORMATION ") ; for adapter in PCI_DEVICES . finalize () . iter () { info ! ("{adapter}") ; } infofooter ! () ; }
};
}
