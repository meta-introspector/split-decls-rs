// Generated macro for NotifCfg (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciNotifCfg {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"NotifCfg"}
// Dependencies: {}
# [doc = " Notification Structure to handle virtqueue notification settings."] # [doc = " See Virtio specification v1.1 - 4.1.4.4"] pub struct NotifCfg { # [doc = " Start addr, from where the notification addresses for the virtqueues are computed"] base_addr : u64 , notify_off_multiplier : u32 , # [doc = " defines the maximum size of the notification space, starting from base_addr."] length : u64 , }
};
}
