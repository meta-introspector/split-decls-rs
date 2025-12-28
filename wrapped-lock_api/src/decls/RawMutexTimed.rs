macro_rules! deps {
    () => {
        RawMutex!();
    };
}

macro_rules! RawMutexTimed {
    () => {
        deps!();
        # [doc = " Additional methods for mutexes which support locking with timeouts."] # [doc = ""] # [doc = " The `Duration` and `Instant` types are specified as associated types so that"] # [doc = " this trait is usable even in `no_std` environments."] pub unsafe trait RawMutexTimed : RawMutex { # [doc = " Duration type used for `try_lock_for`."] type Duration ; # [doc = " Instant type used for `try_lock_until`."] type Instant ; # [doc = " Attempts to acquire this lock until a timeout is reached."] fn try_lock_for (& self , timeout : Self :: Duration) -> bool ; # [doc = " Attempts to acquire this lock until a timeout is reached."] fn try_lock_until (& self , timeout : Self :: Instant) -> bool ; }
    };
}

RawMutexTimed!();