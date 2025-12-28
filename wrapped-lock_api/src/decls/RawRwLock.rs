macro_rules! deps {
    () => {
        GuardSend!();
        GuardNoSend!();
        RwLock!();
    };
}

macro_rules! RawRwLock {
    () => {
        deps!();
        # [doc = " Basic operations for a reader-writer lock."] # [doc = ""] # [doc = " Types implementing this trait can be used by `RwLock` to form a safe and"] # [doc = " fully-functioning `RwLock` type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of this trait must ensure that the `RwLock` is actually"] # [doc = " exclusive: an exclusive lock can't be acquired while an exclusive or shared"] # [doc = " lock exists, and a shared lock can't be acquire while an exclusive lock"] # [doc = " exists."] pub unsafe trait RawRwLock { # [doc = " Initial value for an unlocked `RwLock`."] # [allow (clippy :: declare_interior_mutable_const)] const INIT : Self ; # [doc = " Marker type which determines whether a lock guard should be `Send`. Use"] # [doc = " one of the `GuardSend` or `GuardNoSend` helper types here."] type GuardMarker ; # [doc = " Acquires a shared lock, blocking the current thread until it is able to do so."] fn lock_shared (& self) ; # [doc = " Attempts to acquire a shared lock without blocking."] fn try_lock_shared (& self) -> bool ; # [doc = " Releases a shared lock."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if a shared lock is held in the current context."] unsafe fn unlock_shared (& self) ; # [doc = " Acquires an exclusive lock, blocking the current thread until it is able to do so."] fn lock_exclusive (& self) ; # [doc = " Attempts to acquire an exclusive lock without blocking."] fn try_lock_exclusive (& self) -> bool ; # [doc = " Releases an exclusive lock."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an exclusive lock is held in the current context."] unsafe fn unlock_exclusive (& self) ; # [doc = " Checks if this `RwLock` is currently locked in any way."] # [inline] fn is_locked (& self) -> bool { let acquired_lock = self . try_lock_exclusive () ; if acquired_lock { unsafe { self . unlock_exclusive () ; } } ! acquired_lock } # [doc = " Check if this `RwLock` is currently exclusively locked."] fn is_locked_exclusive (& self) -> bool { let acquired_lock = self . try_lock_shared () ; if acquired_lock { unsafe { self . unlock_shared () ; } } ! acquired_lock } }
    };
}

RawRwLock!()