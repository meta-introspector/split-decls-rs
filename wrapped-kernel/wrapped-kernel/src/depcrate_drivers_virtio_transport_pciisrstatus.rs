// Generated macro for IsrStatus (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciIsrStatus {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"IsrStatus"}
// Dependencies: {}
# [doc = " Wraps a [IsrStatusRaw] in order to preserve"] # [doc = " the original structure and allow interaction with the device via"] # [doc = " the structure."] # [doc = ""] # [doc = " Provides a safe API for Raw structure and allows interaction with the device via"] # [doc = " the structure."] pub struct IsrStatus { # [doc = " References the raw structure in PCI memory space. Is static as"] # [doc = " long as the device is present, which is mandatory in order to let this code work."] isr_stat : VolatileRef < 'static , IsrStatusRaw > , }
};
}
