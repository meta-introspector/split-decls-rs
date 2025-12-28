macro_rules! deps {
    () => {
        Sender!();
        UnboundedSenderInner!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T > Clone for UnboundedSenderInner < T > { fn clone (& self) -> Self { let mut curr = self . inner . num_senders . load (SeqCst) ; loop { if curr == MAX_BUFFER { panic ! ("cannot clone `Sender` -- too many outstanding senders") ; } debug_assert ! (curr < MAX_BUFFER) ; let next = curr + 1 ; match self . inner . num_senders . compare_exchange (curr , next , SeqCst , SeqCst) { Ok (_) => { return Self { inner : self . inner . clone () } ; } Err (actual) => curr = actual , } } } }
    };
}

impl_71!();