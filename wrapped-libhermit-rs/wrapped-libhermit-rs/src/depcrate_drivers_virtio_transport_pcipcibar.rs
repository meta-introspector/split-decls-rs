// Generated macro for PciBar (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciPciBar {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"PciBar"}
// Dependencies: {}
# [doc = " PciBar stores the virtual memory address and associated length of memory space"] # [doc = " a PCI device's physical memory indicated by the device's BAR has been mapped to."] # [derive (Copy , Clone , Debug)] pub struct PciBar { index : u8 , mem_addr : u64 , length : u64 , }
};
}
