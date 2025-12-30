// Generated macro for impl_16 (impl)
macro_rules! Depcrate_mpscimpl_16 {
() => {
// Module: crate::mpsc
// Provides: {"impl_16"}
// Dependencies: {}
impl < T > Drop for Sender < T > { fn drop (& mut self) { let count = Rc :: strong_count (& self . shared) ; let shared = self . shared . borrow_mut () ; if shared . has_receiver && count == 2 { shared . blocked_recv . wake () ; } } }
};
}
