// Generated macro for impl_22 (impl)
macro_rules! Depcrate_notifyimpl_22 {
() => {
// Module: crate::notify
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (feature = "std")] impl < N , T > NotificationPrivate for Tag < N , T > where N : Notification + ? Sized , T : Clone , { type Tag = T ; fn is_additional (& self , i : Internal) -> bool { self . inner . is_additional (i) } fn fence (& self , i : Internal) { self . inner . fence (i) ; } fn count (& self , i : Internal) -> usize { self . inner . count (i) } fn next_tag (& mut self , _ : Internal) -> Self :: Tag { self . tag . clone () } }
};
}
