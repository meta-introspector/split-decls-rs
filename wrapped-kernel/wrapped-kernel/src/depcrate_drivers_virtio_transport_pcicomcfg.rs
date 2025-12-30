// Generated macro for ComCfg (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciComCfg {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"ComCfg"}
// Dependencies: {}
# [doc = " Wraps a [`CommonCfg`] in order to preserve"] # [doc = " the original structure."] # [doc = ""] # [doc = " Provides a safe API for Raw structure and allows interaction with the device via"] # [doc = " the structure."] pub struct ComCfg { # [doc = " References the raw structure in PCI memory space. Is static as"] # [doc = " long as the device is present, which is mandatory in order to let this code work."] com_cfg : VolatileRef < 'static , CommonCfg > , }
};
}
