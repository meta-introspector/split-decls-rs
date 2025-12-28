macro_rules! WordLock {
    () => {
        pub struct WordLock { state : AtomicUsize , }
    };
}

WordLock!();