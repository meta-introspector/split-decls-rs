// Generated macro for impl_592 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedimpl_592 {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"impl_592"}
// Dependencies: {}
impl DrvNotif { # [doc = " Enables notifications by unsetting the LSB."] # [doc = " See Virito specification v1.1. - 2.7.10"] fn enable_notif (& mut self) { self . raw . flags = EventSuppressFlags :: new () . with_desc_event_flags (RingEventFlags :: Enable) ; } # [doc = " Disables notifications by setting the LSB."] # [doc = " See Virtio specification v1.1. - 2.7.10"] fn disable_notif (& mut self) { self . raw . flags = EventSuppressFlags :: new () . with_desc_event_flags (RingEventFlags :: Disable) ; } # [doc = " Enables a notification by the device for a specific descriptor."] fn enable_specific (& mut self , idx : RingIdx) { if self . f_notif_idx { self . raw . flags = EventSuppressFlags :: new () . with_desc_event_flags (RingEventFlags :: Desc) ; self . raw . desc = EventSuppressDesc :: new () . with_desc_event_off (idx . off) . with_desc_event_wrap (idx . wrap) ; } } }
};
}
