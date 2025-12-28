macro_rules! deps {
    () => {
        GuardSend!();
        GuardNoSend!();
        Mutex!();
    };
}

macro_rules! RawMutex {
    () => {
        deps!();
        # [doc = " Basic operations for a mutex."] # [doc = ""] # [doc = " Types implementing this trait can be used by `Mutex` to form a safe and"] # [doc = " fully-functioning mutex type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of this trait must ensure that the mutex is actually"] # [doc = " exclusive: a lock can't be acquired while the mutex is already locked."] pub unsafe trait RawMutex { # [doc = " Initial value for an unlocked mutex."] # [allow (clippy :: declare_interior_mutable_const)] const INIT : Self ; # [doc = " Marker type which determines whether a lock guard should be `Send`. Use"] # [doc = " one of the `GuardSend` or `GuardNoSend` helper types here."] type GuardMarker ; # [doc = " Acquires this mutex, blocking the current thread until it is able to do so."] fn lock (& self) ; # [doc = " Attempts to acquire this mutex without blocking. Returns `true`"] # [doc = " if the lock was successfully acquired and `false` otherwise."] fn try_lock (& self) -> bool ; # [doc = " Unlocks this mutex."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if the mutex is held in the current context, i.e. it must"] # [doc = " be paired with a successful call to [`lock`], [`try_lock`], [`try_lock_for`] or [`try_lock_until`]."] # [doc = ""] # [doc = " [`lock`]: RawMutex::lock"] # [doc = " [`try_lock`]: RawMutex::try_lock"] # [doc = " [`try_lock_for`]: RawMutexTimed::try_lock_for"] # [doc = " [`try_lock_until`]: RawMutexTimed::try_lock_until"] unsafe fn unlock (& self) ; # [doc = " Checks whether the mutex is currently locked."] # [inline] fn is_locked (& self) -> bool { let acquired_lock = self . try_lock () ; if acquired_lock { unsafe { self . unlock () ; } } ! acquired_lock } }
    };
}

RawMutex!();