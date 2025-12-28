macro_rules! deps {
    () => {
        RwLock!();
        RwLockReadGuard!();
        RawRwLockFair!();
        RwLockWriteGuard!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < R : RawRwLockFair , T : ? Sized > RwLock < R , T > { # [doc = " Forcibly unlocks a read lock using a fair unlock protocol."] # [doc = ""] # [doc = " This is useful when combined with `mem::forget` to hold a lock without"] # [doc = " the need to maintain a `RwLockReadGuard` object alive, for example when"] # [doc = " dealing with FFI."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method must only be called if the current thread logically owns a"] # [doc = " `RwLockReadGuard` but that guard has be discarded using `mem::forget`."] # [doc = " Behavior is undefined if a rwlock is read-unlocked when not read-locked."] # [inline] # [track_caller] pub unsafe fn force_unlock_read_fair (& self) { self . raw . unlock_shared_fair () ; } # [doc = " Forcibly unlocks a write lock using a fair unlock protocol."] # [doc = ""] # [doc = " This is useful when combined with `mem::forget` to hold a lock without"] # [doc = " the need to maintain a `RwLockWriteGuard` object alive, for example when"] # [doc = " dealing with FFI."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method must only be called if the current thread logically owns a"] # [doc = " `RwLockWriteGuard` but that guard has be discarded using `mem::forget`."] # [doc = " Behavior is undefined if a rwlock is write-unlocked when not write-locked."] # [inline] # [track_caller] pub unsafe fn force_unlock_write_fair (& self) { self . raw . unlock_exclusive_fair () ; } }
    };
}

impl_113!();