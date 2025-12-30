// Generated macro for impl_551 (impl)
macro_rules! Depcrate_drivers_virtio_transport_pciimpl_551 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"impl_551"}
// Dependencies: {}
impl ShMemCfg { fn new (cap : & PciCap) -> Option < Self > { if cap . bar . length < cap . len () + cap . offset () { error ! ("Shared memory config of with id {} of device {:x}, does not fit into memory specified by bar {:x}!" , cap . cap . id , cap . dev_id , cap . bar . index) ; return None ; } let offset = cap . cap . offset . to_ne () ; let length = cap . cap . length . to_ne () ; let virt_addr_raw = cap . bar . mem_addr + offset ; let raw_ptr = ptr :: with_exposed_provenance_mut :: < u8 > (virt_addr_raw . try_into () . unwrap ()) ; unsafe { for i in 0 .. usize :: try_from (length) . unwrap () { * (raw_ptr . add (i)) = 0 ; } } ; assert ! (mem :: size_of ::< usize > () == 8) ; Some (ShMemCfg { mem_addr : virt_addr_raw , length : cap . len () , sh_mem : ShMem { ptr : raw_ptr , len : cap . bar . length as usize , } , id : cap . cap . id , }) } }
};
}
