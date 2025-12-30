// Generated macro for NetDevCfg (struct)
macro_rules! Depcrate_drivers_net_virtioNetDevCfg {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"NetDevCfg"}
// Dependencies: {}
# [doc = " A wrapper struct for the raw configuration structure."] # [doc = " Handling the right access to fields, as some are read-only"] # [doc = " for the driver."] pub (crate) struct NetDevCfg { pub raw : VolatileRef < 'static , virtio :: net :: Config , ReadOnly > , pub dev_id : u16 , pub features : virtio :: net :: F , }
};
}
