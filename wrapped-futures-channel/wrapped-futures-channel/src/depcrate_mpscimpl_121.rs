// Generated macro for impl_121 (impl)
macro_rules! Depcrate_mpscimpl_121 {
() => {
// Module: crate::mpsc
// Provides: {"impl_121"}
// Dependencies: {}
impl < T > BoundedInner < T > { fn max_senders (& self) -> usize { MAX_CAPACITY - self . buffer } fn set_closed (& self) { let curr = self . state . load (SeqCst) ; if ! decode_state (curr) . is_open { return ; } self . state . fetch_and (! OPEN_MASK , SeqCst) ; } }
};
}
