// Generated macro for read_caps (function)
macro_rules! Depcrate_drivers_virtio_transport_pciread_caps {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"read_caps"}
// Dependencies: {}
# [doc = " Reads all PCI capabilities, starting at the capabilities list pointer from the"] # [doc = " PCI device."] # [doc = ""] # [doc = " Returns ONLY Virtio specific capabilities, which allow to locate the actual capability"] # [doc = " structures inside the memory areas, indicated by the BaseAddressRegisters (BAR's)."] fn read_caps (device : & PciDevice < PciConfigRegion >) -> Result < Vec < PciCap > , PciError > { let device_id = device . device_id () ; let capabilities = device . capabilities () . unwrap () . filter_map (| capability | match capability { PciCapability :: Vendor (capability) => Some (capability) , _ => None , }) . map (| addr | CapData :: read (addr , device . access ()) . unwrap ()) . filter (| cap | cap . cfg_type != CapCfgType :: Pci) . flat_map (| cap | { let slot = cap . bar ; device . memory_map_bar (slot , true) . map (| (addr , size) | PciCap { bar : VirtioPciBar :: new (slot , addr . as_u64 () , size . try_into () . unwrap ()) , dev_id : device_id , cap , }) }) . collect :: < Vec < _ > > () ; if capabilities . is_empty () { error ! ("No virtio capability found for device {device_id:x}") ; Err (PciError :: NoVirtioCaps (device_id)) } else { Ok (capabilities) } }
};
}
