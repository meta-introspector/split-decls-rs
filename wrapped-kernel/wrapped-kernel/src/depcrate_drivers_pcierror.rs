// Generated macro for error (module)
macro_rules! Depcrate_drivers_pcierror {
() => {
// Module: crate::drivers::pci
// Provides: {"error"}
// Dependencies: {}
# [doc = " A module containing PCI specific errors"] # [doc = ""] # [doc = " Errors include..."] pub (crate) mod error { use thiserror :: Error ; # [doc = " An enum of PciErrors"] # [doc = " typically carrying the device's id as an u16."] # [derive (Error , Debug)] pub enum PciError { # [error ("Driver failed to initialize device with id: {0:#x}. Due to unknown reasosn!")] General (u16) , # [error ("Driver failed to initialize device with id: {0:#x}. Reason: No BAR's found.")] NoBar (u16) , # [error ("Driver failed to initialize device with id: {0:#x}. Reason: No Capabilities pointer found.")] NoCapPtr (u16) , # [error ("Driver failed to initialize device with id: {0:#x}. Reason: No Virtio capabilities were found.")] NoVirtioCaps (u16) , } }
};
}
