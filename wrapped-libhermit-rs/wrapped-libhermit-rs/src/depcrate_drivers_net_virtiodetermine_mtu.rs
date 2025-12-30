// Generated macro for determine_mtu (function)
macro_rules! Depcrate_drivers_net_virtiodetermine_mtu {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"determine_mtu"}
// Dependencies: {}
fn determine_mtu (dev_cfg : & NetDevCfg) -> u16 { if dev_cfg . features . contains (virtio :: net :: F :: MTU) { dev_cfg . raw . as_ptr () . mtu () . read () . to_ne () } else { mtu () } }
};
}
