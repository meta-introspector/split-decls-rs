// Generated macro for impl_507 (impl)
macro_rules! Depcrate_drivers_virtio_transport_mmioimpl_507 {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"impl_507"}
// Dependencies: {}
impl NotifCtrl { # [doc = " Returns a new controller. By default MSI-X capabilities and VIRTIO_F_NOTIFICATION_DATA"] # [doc = " are disabled."] pub fn new (notif_addr : * mut le32) -> Self { NotifCtrl { f_notif_data : false , notif_addr , } } # [doc = " Enables VIRTIO_F_NOTIFICATION_DATA. This changes which data is provided to the device. ONLY a good idea if Feature has been negotiated."] pub fn enable_notif_data (& mut self) { self . f_notif_data = true ; } pub fn notify_dev (& self , data : NotificationData) { let notification_data = if self . f_notif_data { data . into_bits () } else { u32 :: from (data . vqn ()) . into () } ; unsafe { self . notif_addr . write_volatile (notification_data) ; } } }
};
}
