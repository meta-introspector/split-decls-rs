// Generated macro for impl_362 (impl)
macro_rules! Depcrate_sync_arcimpl_362 {
() => {
// Module: crate::sync::arc
// Provides: {"impl_362"}
// Dependencies: {}
impl < T : ? Sized > Clone for Arc < T > { # [track_caller] fn clone (& self) -> Arc < T > { self . obj . ref_inc (location ! ()) ; Arc { value : self . value . clone () , obj : self . obj . clone () , } } }
};
}
