// Generated macro for UniCapsColl (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciUniCapsColl {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"UniCapsColl"}
// Dependencies: {}
# [doc = " Universal Caplist Collections holds all universal capability structures for"] # [doc = " a given Virtio PCI device."] # [doc = ""] # [doc = " As Virtio's PCI devices are allowed to present multiple capability"] # [doc = " structures of the same config type, the structure"] # [doc = " provides a driver with all capabilities, sorted in descending priority,"] # [doc = " allowing the driver to choose."] # [doc = " The structure contains a special dev_cfg_list field, a vector holding"] # [doc = " [PciCap] objects, to allow the driver to map its"] # [doc = " device specific configurations independently."] pub struct UniCapsColl { pub (crate) com_cfg : ComCfg , pub (crate) notif_cfg : NotifCfg , pub (crate) isr_cfg : IsrStatus , pub (crate) sh_mem_cfg_list : Vec < ShMemCfg > , pub (crate) dev_cfg_list : Vec < PciCap > , }
};
}
