// Generated macro for impl_118 (impl)
macro_rules! Depcrate_mpscimpl_118 {
() => {
// Module: crate::mpsc
// Provides: {"impl_118"}
// Dependencies: {}
impl < T > Drop for UnboundedReceiver < T > { fn drop (& mut self) { self . close () ; if self . inner . is_some () { loop { match self . next_message () { Poll :: Ready (Some (_)) => { } Poll :: Ready (None) => break , Poll :: Pending => { let state = decode_state (self . inner . as_ref () . unwrap () . state . load (SeqCst)) ; if state . is_closed () { break ; } thread :: yield_now () ; } } } } } }
};
}
