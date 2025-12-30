// Generated macro for network_handler (function)
macro_rules! Depcrate_executor_networknetwork_handler {
() => {
// Module: crate::executor::network
// Provides: {"network_handler"}
// Dependencies: {}
# [cfg (any (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci")) , feature = "rtl8139" , feature = "virtio-net" ,))] pub (crate) fn network_handler () { NIC . lock () . as_nic_mut () . unwrap () . handle_interrupt () ; }
};
}
