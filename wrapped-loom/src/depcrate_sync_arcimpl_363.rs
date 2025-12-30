// Generated macro for impl_363 (impl)
macro_rules! Depcrate_sync_arcimpl_363 {
() => {
// Module: crate::sync::arc
// Provides: {"impl_363"}
// Dependencies: {}
impl < T : ? Sized > Drop for Arc < T > { # [track_caller] fn drop (& mut self) { if self . obj . ref_dec (location ! ()) { assert_eq ! (1 , std :: sync :: Arc :: strong_count (& self . value) , "something odd is going on") ; self . unregister () ; } } }
};
}
