// Generated macro for impl_58 (impl)
macro_rules! Depcrate_address_channelimpl_58 {
() => {
// Module: crate::address::channel
// Provides: {"impl_58"}
// Dependencies: {}
impl < A : Actor > AddressSenderProducer < A > { # [doc = " Are any senders connected"] pub fn connected (& self) -> bool { self . inner . num_senders . load (SeqCst) != 0 } # [doc = " Get channel capacity"] pub fn capacity (& self) -> usize { self . inner . buffer . load (Relaxed) } # [doc = " Set channel capacity"] # [doc = ""] # [doc = " This method wakes up all waiting senders if new capacity is greater"] # [doc = " than current"] pub fn set_capacity (& mut self , cap : usize) { let buffer = self . inner . buffer . load (Relaxed) ; self . inner . buffer . store (cap , Relaxed) ; if cap > buffer { while let Some (task) = unsafe { self . inner . parked_queue . pop_spin () } { task . lock () . notify () ; } } } # [doc = " Get sender side of the channel"] pub fn sender (& self) -> AddressSender < A > { let mut curr = self . inner . num_senders . load (SeqCst) ; loop { if curr == self . inner . max_senders () { panic ! ("cannot clone `Sender` -- too many outstanding senders") ; } let next = curr + 1 ; # [allow (deprecated)] let actual = self . inner . num_senders . compare_and_swap (curr , next , SeqCst) ; if actual == curr { return AddressSender { inner : Arc :: clone (& self . inner) , sender_task : Arc :: new (Mutex :: new (SenderTask :: new ())) , maybe_parked : Arc :: new (AtomicBool :: new (false)) , } ; } curr = actual ; } } }
};
}
