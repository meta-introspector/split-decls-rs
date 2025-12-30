// Generated macro for impl_101 (impl)
macro_rules! Depcrate_incomingimpl_101 {
() => {
// Module: crate::incoming
// Provides: {"impl_101"}
// Dependencies: {}
impl Drop for Incoming { fn drop (& mut self) { if let Some (state) = self . 0 . take () { state . endpoint . refuse (state . inner) ; } } }
};
}
