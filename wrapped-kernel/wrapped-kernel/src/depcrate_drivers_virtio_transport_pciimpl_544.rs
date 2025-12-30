// Generated macro for impl_544 (impl)
macro_rules! Depcrate_drivers_virtio_transport_pciimpl_544 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"impl_544"}
// Dependencies: {}
impl NotifCfg { fn new (cap : & PciCap) -> Option < Self > { if cap . bar . length < cap . len () + cap . offset () { let dev_id = cap . dev_id ; let index = cap . bar . index ; error ! ("Notification config of device {dev_id:x}, does not fit into memory specified by bar {index:x}!") ; return None ; } let notify_off_multiplier = cap . cap . notify_off_multiplier ? . to_ne () ; let base_addr = cap . bar . mem_addr + cap . offset () ; Some (NotifCfg { base_addr , notify_off_multiplier , length : cap . len () , }) } pub fn notification_location (& self , vq_cfg_handler : & mut VqCfgHandler < '_ >) -> * mut le32 { let addend = u32 :: from (vq_cfg_handler . notif_off ()) * self . notify_off_multiplier ; let addr = self . base_addr + u64 :: from (addend) ; ptr :: with_exposed_provenance_mut (addr . try_into () . unwrap ()) } }
};
}
