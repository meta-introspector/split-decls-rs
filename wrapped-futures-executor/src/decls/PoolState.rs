macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! PoolState {
    () => {
        deps!();
        struct PoolState { tx : Mutex < mpsc :: Sender < Message > > , rx : Mutex < mpsc :: Receiver < Message > > , cnt : AtomicUsize , size : usize , }
    };
}

PoolState!();