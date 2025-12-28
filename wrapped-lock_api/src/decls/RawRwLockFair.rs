macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RawRwLockFair {
    () => {
        deps!();
        # [doc = " Additional methods for `RwLock`s which support fair unlocking."] # [doc = ""] # [doc = " Fair unlocking means that a lock is handed directly over to the next waiting"] # [doc = " thread if there is one, without giving other threads the opportunity to"] # [doc = " \"steal\" the lock in the meantime. This is typically slower than unfair"] # [doc = " unlocking, but may be necessary in certain circumstances."] pub unsafe trait RawRwLockFair : RawRwLock { # [doc = " Releases a shared lock using a fair unlock protocol."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if a shared lock is held in the current context."] unsafe fn unlock_shared_fair (& self) ; # [doc = " Releases an exclusive lock using a fair unlock protocol."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an exclusive lock is held in the current context."] unsafe fn unlock_exclusive_fair (& self) ; # [doc = " Temporarily yields a shared lock to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_shared_fair` followed"] # [doc = " by `lock_shared`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if a shared lock is held in the current context."] unsafe fn bump_shared (& self) { self . unlock_shared_fair () ; self . lock_shared () ; } # [doc = " Temporarily yields an exclusive lock to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_exclusive_fair` followed"] # [doc = " by `lock_exclusive`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an exclusive lock is held in the current context."] unsafe fn bump_exclusive (& self) { self . unlock_exclusive_fair () ; self . lock_exclusive () ; } }
    };
}

RawRwLockFair!()