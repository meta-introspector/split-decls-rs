// Generated macro for impl_51 (impl)
macro_rules! Depcrate_address_channelimpl_51 {
() => {
// Module: crate::address::channel
// Provides: {"impl_51"}
// Dependencies: {}
impl < A : Actor > Clone for AddressSender < A > { fn clone (& self) -> AddressSender < A > { let mut curr = self . inner . num_senders . load (SeqCst) ; loop { if curr == self . inner . max_senders () { panic ! ("cannot clone `Sender` -- too many outstanding senders") ; } debug_assert ! (curr < self . inner . max_senders ()) ; let next = curr + 1 ; # [allow (deprecated)] let actual = self . inner . num_senders . compare_and_swap (curr , next , SeqCst) ; if actual == curr { return AddressSender { inner : Arc :: clone (& self . inner) , sender_task : Arc :: new (Mutex :: new (SenderTask :: new ())) , maybe_parked : Arc :: new (AtomicBool :: new (false)) , } ; } curr = actual ; } } }
};
}
