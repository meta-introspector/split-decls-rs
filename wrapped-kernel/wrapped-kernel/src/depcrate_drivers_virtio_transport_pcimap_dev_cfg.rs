// Generated macro for map_dev_cfg (function)
macro_rules! Depcrate_drivers_virtio_transport_pcimap_dev_cfg {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"map_dev_cfg"}
// Dependencies: {}
# [doc = " Maps a given device specific pci configuration structure and"] # [doc = " returns a static reference to it."] pub fn map_dev_cfg < T > (cap : & PciCap) -> Option < & 'static mut T > { if cap . cap . cfg_type != CapCfgType :: Device { error ! ("Capability of device config has wrong id. Mapping not possible...") ; return None ; } ; if cap . bar_len () < cap . len () + cap . offset () { error ! ("Device config of device {:x}, does not fit into memory specified by bar!" , cap . dev_id () ,) ; return None ; } if cap . len () < u64 :: try_from (mem :: size_of :: < T > ()) . unwrap () { error ! ("Device specific config from device {:x}, does not represent actual structure specified by the standard!" , cap . dev_id ()) ; return None ; } let virt_addr_raw = cap . bar_addr () + cap . offset () ; let dev_cfg : & 'static mut T = unsafe { & mut * (ptr :: with_exposed_provenance_mut (virt_addr_raw . try_into () . unwrap ())) } ; Some (dev_cfg) }
};
}
