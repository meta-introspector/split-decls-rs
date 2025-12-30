// Generated macro for impl_29 (impl)
macro_rules! Depcrate_notifyimpl_29 {
() => {
// Module: crate::notify
// Provides: {"impl_29"}
// Dependencies: {}
impl < T , F : TagProducer < Tag = T > > NotificationPrivate for GenericNotify < F > { type Tag = T ; fn is_additional (& self , _ : Internal) -> bool { self . additional } fn fence (& self , _ : Internal) { } fn count (& self , _ : Internal) -> usize { self . count } fn next_tag (& mut self , _ : Internal) -> Self :: Tag { self . tags . next_tag () } }
};
}
