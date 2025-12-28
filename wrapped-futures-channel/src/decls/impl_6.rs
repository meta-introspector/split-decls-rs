macro_rules! deps {
    () => {
        TryLock!();
        Lock!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T > Lock < T > { # [doc = " Creates a new lock around the given value."] pub (crate) fn new (t : T) -> Self { Self { locked : AtomicBool :: new (false) , data : UnsafeCell :: new (t) } } # [doc = " Attempts to acquire this lock, returning whether the lock was acquired or"] # [doc = " not."] # [doc = ""] # [doc = " If `Some` is returned then the data this lock protects can be accessed"] # [doc = " through the sentinel. This sentinel allows both mutable and immutable"] # [doc = " access."] # [doc = ""] # [doc = " If `None` is returned then the lock is already locked, either elsewhere"] # [doc = " on this thread or on another thread."] pub (crate) fn try_lock (& self) -> Option < TryLock < '_ , T > > { if ! self . locked . swap (true , SeqCst) { Some (TryLock { __ptr : self }) } else { None } } }
    };
}

impl_6!();