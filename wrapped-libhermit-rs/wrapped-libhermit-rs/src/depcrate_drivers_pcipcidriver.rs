// Generated macro for PciDriver (enum)
macro_rules! Depcrate_drivers_pciPciDriver {
() => {
// Module: crate::drivers::pci
// Provides: {"PciDriver"}
// Dependencies: {}
# [allow (clippy :: large_enum_variant)] # [allow (clippy :: enum_variant_names)] # [non_exhaustive] pub (crate) enum PciDriver { # [cfg (feature = "fuse")] VirtioFs (InterruptTicketMutex < VirtioFsDriver >) , # [cfg (feature = "console")] VirtioConsole (InterruptTicketMutex < VirtioConsoleDriver >) , # [cfg (feature = "vsock")] VirtioVsock (InterruptTicketMutex < VirtioVsockDriver >) , }
};
}
