// Generated macro for impl_919 (impl)
macro_rules! Depcrateimpl_919 {
() => {
// Module: crate
// Provides: {"impl_919"}
// Dependencies: {}
impl < F : FnOnce () > OnDrop < F > { # [doc = " Disables on-drop call."] # [inline] pub fn disable (mut self) { self . 0 . take () ; } }
};
}
