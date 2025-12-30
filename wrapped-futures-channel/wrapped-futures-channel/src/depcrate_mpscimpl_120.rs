// Generated macro for impl_120 (impl)
macro_rules! Depcrate_mpscimpl_120 {
() => {
// Module: crate::mpsc
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > UnboundedInner < T > { fn set_closed (& self) { let curr = self . state . load (SeqCst) ; if ! decode_state (curr) . is_open { return ; } self . state . fetch_and (! OPEN_MASK , SeqCst) ; } }
};
}
