// Generated macro for impl_155 (impl)
macro_rules! Depcrate_common_watchimpl_155 {
() => {
// Module: crate::common::watch
// Provides: {"impl_155"}
// Dependencies: {}
impl Receiver { pub (crate) fn load (& mut self , cx : & mut task :: Context < '_ >) -> Value { self . shared . waker . register (cx . waker ()) ; self . shared . value . load (Ordering :: SeqCst) } pub (crate) fn peek (& self) -> Value { self . shared . value . load (Ordering :: Relaxed) } }
};
}
