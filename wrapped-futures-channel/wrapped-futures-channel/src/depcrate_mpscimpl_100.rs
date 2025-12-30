// Generated macro for impl_100 (impl)
macro_rules! Depcrate_mpscimpl_100 {
() => {
// Module: crate::mpsc
// Provides: {"impl_100"}
// Dependencies: {}
impl < T > Clone for BoundedSenderInner < T > { fn clone (& self) -> Self { let mut curr = self . inner . num_senders . load (SeqCst) ; loop { if curr == self . inner . max_senders () { panic ! ("cannot clone `Sender` -- too many outstanding senders") ; } debug_assert ! (curr < self . inner . max_senders ()) ; let next = curr + 1 ; match self . inner . num_senders . compare_exchange (curr , next , SeqCst , SeqCst) { Ok (_) => { return Self { inner : self . inner . clone () , sender_task : Arc :: new (Mutex :: new (SenderTask :: new ())) , maybe_parked : false , } ; } Err (actual) => curr = actual , } } } }
};
}
