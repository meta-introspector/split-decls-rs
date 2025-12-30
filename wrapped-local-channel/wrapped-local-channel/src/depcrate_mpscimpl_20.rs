// Generated macro for impl_20 (impl)
macro_rules! Depcrate_mpscimpl_20 {
() => {
// Module: crate::mpsc
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > Stream for Receiver < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut shared = self . shared . borrow_mut () ; if Rc :: strong_count (& self . shared) == 1 { return Poll :: Ready (shared . buffer . pop_front ()) ; } if let Some (msg) = shared . buffer . pop_front () { Poll :: Ready (Some (msg)) } else { shared . blocked_recv . register (cx . waker ()) ; Poll :: Pending } } }
};
}
