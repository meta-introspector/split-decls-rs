// Generated macro for impl_547 (impl)
macro_rules! Depcrate_drivers_virtio_transport_pciimpl_547 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"impl_547"}
// Dependencies: {}
impl NotifCtrl { # [doc = " Returns a new controller. By default MSI-X capabilities and VIRTIO_F_NOTIFICATION_DATA"] # [doc = " are disabled."] pub fn new (notif_addr : * mut le32) -> Self { NotifCtrl { f_notif_data : false , notif_addr , } } # [doc = " Enables VIRTIO_F_NOTIFICATION_DATA. This changes which data is provided to the device. ONLY a good idea if Feature has been negotiated."] pub fn enable_notif_data (& mut self) { self . f_notif_data = true ; } pub fn notify_dev (& self , data : NotificationData) { if self . f_notif_data { unsafe { self . notif_addr . write_volatile (data . into_bits ()) ; } } else { unsafe { self . notif_addr . cast :: < le16 > () . write_volatile (data . vqn () . into ()) ; } } } }
};
}
