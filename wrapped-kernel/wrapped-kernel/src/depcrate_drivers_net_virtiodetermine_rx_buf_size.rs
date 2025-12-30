// Generated macro for determine_rx_buf_size (function)
macro_rules! Depcrate_drivers_net_virtiodetermine_rx_buf_size {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"determine_rx_buf_size"}
// Dependencies: {}
fn determine_rx_buf_size (dev_cfg : & NetDevCfg) -> u32 { let mut min_buf_size = determine_mtu (dev_cfg) . into () ; if dev_cfg . features . contains (virtio :: net :: F :: MRG_RXBUF) && let Some (my_mrg_rxbuf_size) = hermit_var ! ("HERMIT_MRG_RXBUF_SIZE") { let my_mrg_rxbuf_size = u32 :: from_str (& my_mrg_rxbuf_size) . unwrap () ; assert ! (my_mrg_rxbuf_size > 0 , "VIRTIO does not allow buffer elements of size 0.") ; min_buf_size = my_mrg_rxbuf_size ; } else { if dev_cfg . features . contains (virtio :: net :: F :: GUEST_TSO4) || dev_cfg . features . contains (virtio :: net :: F :: GUEST_TSO6) || dev_cfg . features . contains (virtio :: net :: F :: GUEST_UFO) { min_buf_size = u32 :: max (min_buf_size , 65562 - size_of :: < Hdr > () as u32) ; } else { min_buf_size = u32 :: max (min_buf_size , 1526 - size_of :: < Hdr > () as u32) ; } } min_buf_size }
};
}
