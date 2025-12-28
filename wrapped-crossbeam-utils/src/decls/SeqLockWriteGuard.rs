macro_rules! deps {
    () => {
        SeqLock!();
    };
}

macro_rules! SeqLockWriteGuard {
    () => {
        deps!();
        # [doc = " An RAII guard that releases the lock and increments the stamp when dropped."] pub (crate) struct SeqLockWriteGuard { # [doc = " The parent lock."] lock : & 'static SeqLock , # [doc = " The stamp before locking."] state : usize , }
    };
}

SeqLockWriteGuard!();