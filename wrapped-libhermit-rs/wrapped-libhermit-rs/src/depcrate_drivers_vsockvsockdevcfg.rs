// Generated macro for VsockDevCfg (struct)
macro_rules! Depcrate_drivers_vsockVsockDevCfg {
() => {
// Module: crate::drivers::vsock
// Provides: {"VsockDevCfg"}
// Dependencies: {}
# [doc = " A wrapper struct for the raw configuration structure."] # [doc = " Handling the right access to fields, as some are read-only"] # [doc = " for the driver."] pub (crate) struct VsockDevCfg { pub raw : & 'static VsockDevCfgRaw , pub dev_id : u16 , pub features : virtio :: vsock :: F , }
};
}
