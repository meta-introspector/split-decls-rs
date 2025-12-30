// Generated macro for impl_665 (impl)
macro_rules! Depcrate_interpret_stackimpl_665 {
() => {
// Module: crate::interpret::stack
// Provides: {"impl_665"}
// Dependencies: {}
impl Drop for SpanGuard { fn drop (& mut self) { self . 0 . with_subscriber (| (id , dispatch) | { dispatch . exit (id) ; }) ; } }
};
}
