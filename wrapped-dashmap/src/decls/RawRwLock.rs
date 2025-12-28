macro_rules! RawRwLock {
    () => {
        pub struct RawRwLock { state : AtomicUsize , }
    };
}

RawRwLock!()