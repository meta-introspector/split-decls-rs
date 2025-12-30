// Generated macro for PciCap (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciPciCap {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"PciCap"}
// Dependencies: {}
# [doc = " Virtio's PCI capabilities structure."] # [doc = " See Virtio specification v.1.1 - 4.1.4"] # [doc = ""] # [doc = " Indicating: Where the capability field is mapped in memory and"] # [doc = " Which id (sometimes also indicates priority for multiple"] # [doc = " capabilities of same type) it holds."] # [doc = ""] # [doc = " This structure does NOT represent the structure in the standard,"] # [doc = " as it is not directly mapped into address space from PCI device"] # [doc = " configuration space."] # [doc = " Therefore the struct only contains necessary information to map"] # [doc = " corresponding config type into address space."] # [derive (Clone)] pub struct PciCap { bar : PciBar , dev_id : u16 , cap : CapData , }
};
}
