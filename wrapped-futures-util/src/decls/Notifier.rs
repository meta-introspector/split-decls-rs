macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! Notifier {
    () => {
        deps!();
        struct Notifier { state : AtomicUsize , wakers : Mutex < Option < Slab < Option < Waker > > > > , }
    };
}

Notifier!()