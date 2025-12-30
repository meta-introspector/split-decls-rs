// Generated macro for ShMemCfg (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciShMemCfg {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"ShMemCfg"}
// Dependencies: {}
# [doc = " Shared memory configuration structure of Virtio PCI devices."] # [doc = " See Virtio specification v1.1. - 4.1.4.7"] # [doc = ""] # [doc = " Each shared memory region is defined via a single shared"] # [doc = " memory structure. Each region is identified by an id indicated"] # [doc = " via the capability.id field of PciCapRaw."] # [doc = ""] # [doc = " The shared memory region is defined via a PciCap64 structure."] # [doc = " See Virtio specification v.1.1 - 4.1.4 for structure."] # [doc = ""] pub struct ShMemCfg { mem_addr : u64 , length : u64 , sh_mem : ShMem , # [doc = " Shared memory regions are identified via an ID"] # [doc = " See Virtio specification v1.1. - 4.1.4.7"] id : u8 , }
};
}
