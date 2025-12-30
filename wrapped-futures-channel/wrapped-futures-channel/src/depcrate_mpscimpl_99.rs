// Generated macro for impl_99 (impl)
macro_rules! Depcrate_mpscimpl_99 {
() => {
// Module: crate::mpsc
// Provides: {"impl_99"}
// Dependencies: {}
impl < T > Clone for UnboundedSenderInner < T > { fn clone (& self) -> Self { let mut curr = self . inner . num_senders . load (SeqCst) ; loop { if curr == MAX_BUFFER { panic ! ("cannot clone `Sender` -- too many outstanding senders") ; } debug_assert ! (curr < MAX_BUFFER) ; let next = curr + 1 ; match self . inner . num_senders . compare_exchange (curr , next , SeqCst , SeqCst) { Ok (_) => { return Self { inner : self . inner . clone () } ; } Err (actual) => curr = actual , } } } }
};
}
