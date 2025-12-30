// Generated macro for impl_13 (impl)
macro_rules! Depcrate_notifyimpl_13 {
() => {
// Module: crate::notify
// Provides: {"impl_13"}
// Dependencies: {}
impl NotificationPrivate for Notify { type Tag = () ; fn is_additional (& self , _ : Internal) -> bool { false } fn fence (& self , _ : Internal) { full_fence () ; } fn count (& self , _ : Internal) -> usize { self . 0 } fn next_tag (& mut self , _ : Internal) -> Self :: Tag { } }
};
}
