macro_rules! deps {
    () => {
        SenderTask!();
        BoundedSenderInner!();
        Sender!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T > Clone for BoundedSenderInner < T > { fn clone (& self) -> Self { let mut curr = self . inner . num_senders . load (SeqCst) ; loop { if curr == self . inner . max_senders () { panic ! ("cannot clone `Sender` -- too many outstanding senders") ; } debug_assert ! (curr < self . inner . max_senders ()) ; let next = curr + 1 ; match self . inner . num_senders . compare_exchange (curr , next , SeqCst , SeqCst) { Ok (_) => { return Self { inner : self . inner . clone () , sender_task : Arc :: new (Mutex :: new (SenderTask :: new ())) , maybe_parked : false , } ; } Err (actual) => curr = actual , } } } }
    };
}

impl_72!();