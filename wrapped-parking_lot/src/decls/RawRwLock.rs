macro_rules! RawRwLock {
    () => {
        # [doc = " Raw reader-writer lock type backed by the parking lot."] pub struct RawRwLock { state : AtomicUsize , }
    };
}

RawRwLock!();