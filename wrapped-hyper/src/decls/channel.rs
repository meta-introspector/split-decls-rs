macro_rules! deps {
    () => {
        Receiver!();
        Shared!();
        Sender!();
        Value!();
    };
}

macro_rules! channel {
    () => {
        deps!();
        pub (crate) fn channel (initial : Value) -> (Sender , Receiver) { debug_assert ! (initial != CLOSED , "watch::channel initial state of 0 is reserved") ; let shared = Arc :: new (Shared { value : AtomicUsize :: new (initial) , waker : AtomicWaker :: new () , }) ; (Sender { shared : shared . clone () , } , Receiver { shared } ,) }
    };
}

channel!()