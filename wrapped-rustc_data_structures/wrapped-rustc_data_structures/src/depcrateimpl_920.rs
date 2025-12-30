// Generated macro for impl_920 (impl)
macro_rules! Depcrateimpl_920 {
() => {
// Module: crate
// Provides: {"impl_920"}
// Dependencies: {}
impl < F : FnOnce () > Drop for OnDrop < F > { # [inline] fn drop (& mut self) { if let Some (f) = self . 0 . take () { f () ; } } }
};
}
