// Generated macro for impl_9 (impl)
macro_rules! Depcrate_contextimpl_9 {
() => {
// Module: crate::context
// Provides: {"impl_9"}
// Dependencies: {}
impl Drop for Context { fn drop (& mut self) { if let Some (sender) = self . send_on_drop . take () { let _ = self . flush_messages (& * sender . 0) ; } } }
};
}
