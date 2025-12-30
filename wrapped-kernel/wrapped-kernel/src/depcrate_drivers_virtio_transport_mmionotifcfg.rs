// Generated macro for NotifCfg (struct)
macro_rules! Depcrate_drivers_virtio_transport_mmioNotifCfg {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"NotifCfg"}
// Dependencies: {}
# [doc = " Notification Structure to handle virtqueue notification settings."] # [doc = " See Virtio specification v1.1 - 4.1.4.4"] pub struct NotifCfg { # [doc = " Start addr, from where the notification addresses for the virtqueues are computed"] queue_notify : * mut le32 , }
};
}
