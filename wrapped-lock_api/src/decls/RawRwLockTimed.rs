macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RawRwLockTimed {
    () => {
        deps!();
        # [doc = " Additional methods for `RwLock`s which support locking with timeouts."] # [doc = ""] # [doc = " The `Duration` and `Instant` types are specified as associated types so that"] # [doc = " this trait is usable even in `no_std` environments."] pub unsafe trait RawRwLockTimed : RawRwLock { # [doc = " Duration type used for `try_lock_for`."] type Duration ; # [doc = " Instant type used for `try_lock_until`."] type Instant ; # [doc = " Attempts to acquire a shared lock until a timeout is reached."] fn try_lock_shared_for (& self , timeout : Self :: Duration) -> bool ; # [doc = " Attempts to acquire a shared lock until a timeout is reached."] fn try_lock_shared_until (& self , timeout : Self :: Instant) -> bool ; # [doc = " Attempts to acquire an exclusive lock until a timeout is reached."] fn try_lock_exclusive_for (& self , timeout : Self :: Duration) -> bool ; # [doc = " Attempts to acquire an exclusive lock until a timeout is reached."] fn try_lock_exclusive_until (& self , timeout : Self :: Instant) -> bool ; }
    };
}

RawRwLockTimed!()