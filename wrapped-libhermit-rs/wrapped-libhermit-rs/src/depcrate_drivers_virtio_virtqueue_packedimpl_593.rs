// Generated macro for impl_593 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedimpl_593 {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"impl_593"}
// Dependencies: {}
impl DevNotif { # [doc = " Enables the notification capability for a specific buffer."] pub fn enable_notif_specific (& mut self) { self . f_notif_idx = true ; } # [doc = " Reads notification bit (i.e. LSB) and returns value."] # [doc = " If notifications are enabled returns true, else false."] fn is_notif (& self) -> bool { self . raw . flags . desc_event_flags () == RingEventFlags :: Enable } fn notif_specific (& self) -> Option < RingIdx > { if ! self . f_notif_idx { return None ; } if self . raw . flags . desc_event_flags () != RingEventFlags :: Desc { return None ; } let off = self . raw . desc . desc_event_off () ; let wrap = self . raw . desc . desc_event_wrap () ; Some (RingIdx { off , wrap }) } }
};
}
