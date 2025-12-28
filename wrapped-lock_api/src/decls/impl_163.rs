macro_rules! deps {
    () => {
        RawRwLockUpgradeFair!();
        RwLockUpgradableReadGuard!();
        RwLock!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgradeFair + 'a , T : ? Sized + 'a > RwLockUpgradableReadGuard < 'a , R , T > { # [doc = " Unlocks the `RwLock` using a fair unlock protocol."] # [doc = ""] # [doc = " By default, `RwLock` is unfair and allow the current thread to re-lock"] # [doc = " the `RwLock` before another has the chance to acquire the lock, even if"] # [doc = " that thread has been blocked on the `RwLock` for a long time. This is"] # [doc = " the default because it allows much higher throughput as it avoids"] # [doc = " forcing a context switch on every `RwLock` unlock. This can result in one"] # [doc = " thread acquiring a `RwLock` many more times than other threads."] # [doc = ""] # [doc = " However in some cases it can be beneficial to ensure fairness by forcing"] # [doc = " the lock to pass on to a waiting thread if there is one. This is done by"] # [doc = " using this method instead of dropping the `RwLockUpgradableReadGuard` normally."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { unsafe { s . rwlock . raw . unlock_upgradable_fair () ; } mem :: forget (s) ; } # [doc = " Temporarily unlocks the `RwLock` to execute the given function."] # [doc = ""] # [doc = " The `RwLock` is unlocked a fair unlock protocol."] # [doc = ""] # [doc = " This is safe because `&mut` guarantees that there exist no other"] # [doc = " references to the data protected by the `RwLock`."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . rwlock . raw . unlock_upgradable_fair () ; } defer ! (s . rwlock . raw . lock_upgradable ()) ; f () } # [doc = " Temporarily yields the `RwLock` to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_fair` followed"] # [doc = " by `upgradable_read`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . rwlock . raw . bump_upgradable () ; } } }
    };
}

impl_163!();