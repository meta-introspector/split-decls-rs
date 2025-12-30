// Generated macro for impl_16 (impl)
macro_rules! Depcrate_notifyimpl_16 {
() => {
// Module: crate::notify
// Provides: {"impl_16"}
// Dependencies: {}
impl < N > NotificationPrivate for Additional < N > where N : Notification + ? Sized , { type Tag = N :: Tag ; fn is_additional (& self , _ : Internal) -> bool { true } fn fence (& self , i : Internal) { self . 0 . fence (i) ; } fn count (& self , i : Internal) -> usize { self . 0 . count (i) } fn next_tag (& mut self , i : Internal) -> Self :: Tag { self . 0 . next_tag (i) } }
};
}
