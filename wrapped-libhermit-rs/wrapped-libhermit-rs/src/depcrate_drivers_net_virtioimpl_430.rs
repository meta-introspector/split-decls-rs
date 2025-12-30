// Generated macro for impl_430 (impl)
macro_rules! Depcrate_drivers_net_virtioimpl_430 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"impl_430"}
// Dependencies: {}
impl NetworkDriver for VirtioNetDriver < Init > { # [doc = " Returns the mac address of the device."] # [doc = " If VIRTIO_NET_F_MAC is not set, the function panics currently!"] fn get_mac_address (& self) -> [u8 ; 6] { if self . dev_cfg . features . contains (virtio :: net :: F :: MAC) { self . com_cfg . device_config_space () . read_config_with (| | self . dev_cfg . raw . as_ptr () . mac () . read ()) } else { unreachable ! ("Currently VIRTIO_NET_F_MAC must be negotiated!") } } # [allow (dead_code)] fn has_packet (& self) -> bool { self . inner . recv_vqs . has_packet () } fn set_polling_mode (& mut self , value : bool) { if value { self . disable_interrupts () ; } else { self . enable_interrupts () ; } } fn handle_interrupt (& mut self) { let status = self . isr_stat . is_queue_interrupt () ; # [cfg (not (feature = "pci"))] if status . contains (virtio :: mmio :: InterruptStatus :: CONFIGURATION_CHANGE_NOTIFICATION) { info ! ("Configuration changes are not possible! Aborting") ; todo ! ("Implement possibility to change config on the fly...") } # [cfg (feature = "pci")] if status . contains (virtio :: pci :: IsrStatus :: DEVICE_CONFIGURATION_INTERRUPT) { info ! ("Configuration changes are not possible! Aborting") ; todo ! ("Implement possibility to change config on the fly...") } self . isr_stat . acknowledge () ; } }
};
}
