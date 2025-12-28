macro_rules! deps {
    () => {
        Receiver!();
        Counter!();
        Sender!();
    };
}

macro_rules! new {
    () => {
        deps!();
        # [doc = " Wraps a channel into the reference counter."] pub (crate) fn new < C > (chan : C) -> (Sender < C > , Receiver < C >) { let counter = NonNull :: from (Box :: leak (Box :: new (Counter { senders : AtomicUsize :: new (1) , receivers : AtomicUsize :: new (1) , destroy : AtomicBool :: new (false) , chan , }))) ; let s = Sender { counter } ; let r = Receiver { counter } ; (s , r) }
    };
}

new!();