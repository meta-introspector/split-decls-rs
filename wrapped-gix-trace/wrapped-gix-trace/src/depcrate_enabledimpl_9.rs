// Generated macro for impl_9 (impl)
macro_rules! Depcrate_enabledimpl_9 {
() => {
// Module: crate::enabled
// Provides: {"impl_9"}
// Dependencies: {}
impl Drop for Span { fn drop (& mut self) { if let Some ((id , dispatch , _meta)) = self . id . take () { dispatch . exit (& id) ; dispatch . try_close (id) ; } } }
};
}
