macro_rules! SeqLock {
    () => {
        # [doc = " A simple stamped lock."] pub (crate) struct SeqLock { # [doc = " The current state of the lock."] # [doc = ""] # [doc = " All bits except the least significant one hold the current stamp. When locked, the state"] # [doc = " equals 1 and doesn't contain a valid stamp."] state : AtomicUsize , }
    };
}

SeqLock!()