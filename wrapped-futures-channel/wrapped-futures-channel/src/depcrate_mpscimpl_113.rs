// Generated macro for impl_113 (impl)
macro_rules! Depcrate_mpscimpl_113 {
() => {
// Module: crate::mpsc
// Provides: {"impl_113"}
// Dependencies: {}
impl < T > Drop for Receiver < T > { fn drop (& mut self) { self . close () ; if self . inner . is_some () { loop { match self . next_message () { Poll :: Ready (Some (_)) => { } Poll :: Ready (None) => break , Poll :: Pending => { let state = decode_state (self . inner . as_ref () . unwrap () . state . load (SeqCst)) ; if state . is_closed () { break ; } thread :: yield_now () ; } } } } } }
};
}
