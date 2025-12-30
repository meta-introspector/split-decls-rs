// Generated macro for impl_19 (impl)
macro_rules! Depcrate_notifyimpl_19 {
() => {
// Module: crate::notify
// Provides: {"impl_19"}
// Dependencies: {}
impl < N > NotificationPrivate for Relaxed < N > where N : Notification + ? Sized , { type Tag = N :: Tag ; fn is_additional (& self , i : Internal) -> bool { self . 0 . is_additional (i) } fn fence (& self , _ : Internal) { } fn count (& self , i : Internal) -> usize { self . 0 . count (i) } fn next_tag (& mut self , i : Internal) -> Self :: Tag { self . 0 . next_tag (i) } }
};
}
