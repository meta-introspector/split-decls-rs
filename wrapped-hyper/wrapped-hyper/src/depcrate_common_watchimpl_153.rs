// Generated macro for impl_153 (impl)
macro_rules! Depcrate_common_watchimpl_153 {
() => {
// Module: crate::common::watch
// Provides: {"impl_153"}
// Dependencies: {}
impl Sender { pub (crate) fn send (& mut self , value : Value) { if self . shared . value . swap (value , Ordering :: SeqCst) != value { self . shared . waker . wake () ; } } }
};
}
