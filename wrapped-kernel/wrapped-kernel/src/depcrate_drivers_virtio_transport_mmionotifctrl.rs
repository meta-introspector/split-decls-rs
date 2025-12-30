// Generated macro for NotifCtrl (struct)
macro_rules! Depcrate_drivers_virtio_transport_mmioNotifCtrl {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"NotifCtrl"}
// Dependencies: {}
# [doc = " Control structure, allowing to notify a device via PCI bus."] # [doc = " Typically hold by a virtqueue."] pub struct NotifCtrl { # [doc = " Indicates if VIRTIO_F_NOTIFICATION_DATA has been negotiated"] f_notif_data : bool , # [doc = " Where to write notification"] notif_addr : * mut le32 , }
};
}
