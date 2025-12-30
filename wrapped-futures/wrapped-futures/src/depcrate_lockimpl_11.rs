// Generated macro for impl_11 (impl)
macro_rules! Depcrate_lockimpl_11 {
() => {
// Module: crate::lock
// Provides: {"impl_11"}
// Dependencies: {}
impl < T > Lock < T > { # [doc = " Creates a new lock around the given value."] pub fn new (t : T) -> Lock < T > { Lock { locked : AtomicBool :: new (false) , data : UnsafeCell :: new (t) , } } # [doc = " Attempts to acquire this lock, returning whether the lock was acquired or"] # [doc = " not."] # [doc = ""] # [doc = " If `Some` is returned then the data this lock protects can be accessed"] # [doc = " through the sentinel. This sentinel allows both mutable and immutable"] # [doc = " access."] # [doc = ""] # [doc = " If `None` is returned then the lock is already locked, either elsewhere"] # [doc = " on this thread or on another thread."] pub fn try_lock (& self) -> Option < TryLock < T > > { if ! self . locked . swap (true , Acquire) { Some (TryLock { __ptr : self }) } else { None } } }
};
}
