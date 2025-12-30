// Generated macro for impl_26 (impl)
macro_rules! Depcrate_notifyimpl_26 {
() => {
// Module: crate::notify
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "std")] impl < N , F , T > NotificationPrivate for TagWith < N , F > where N : Notification + ? Sized , F : FnMut () -> T , { type Tag = T ; fn is_additional (& self , i : Internal) -> bool { self . inner . is_additional (i) } fn fence (& self , i : Internal) { self . inner . fence (i) ; } fn count (& self , i : Internal) -> usize { self . inner . count (i) } fn next_tag (& mut self , _ : Internal) -> Self :: Tag { (self . tag) () } }
};
}
