macro_rules! deps {
    () => {
        MutexGuard!();
        RawMutexFair!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a , R : RawMutexFair + 'a , T : ? Sized + 'a > MutexGuard < 'a , R , T > { # [doc = " Unlocks the mutex using a fair unlock protocol."] # [doc = ""] # [doc = " By default, mutexes are unfair and allow the current thread to re-lock"] # [doc = " the mutex before another has the chance to acquire the lock, even if"] # [doc = " that thread has been blocked on the mutex for a long time. This is the"] # [doc = " default because it allows much higher throughput as it avoids forcing a"] # [doc = " context switch on every mutex unlock. This can result in one thread"] # [doc = " acquiring a mutex many more times than other threads."] # [doc = ""] # [doc = " However in some cases it can be beneficial to ensure fairness by forcing"] # [doc = " the lock to pass on to a waiting thread if there is one. This is done by"] # [doc = " using this method instead of dropping the `MutexGuard` normally."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { unsafe { s . mutex . raw . unlock_fair () ; } mem :: forget (s) ; } # [doc = " Temporarily unlocks the mutex to execute the given function."] # [doc = ""] # [doc = " The mutex is unlocked using a fair unlock protocol."] # [doc = ""] # [doc = " This is safe because `&mut` guarantees that there exist no other"] # [doc = " references to the data protected by the mutex."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . mutex . raw . unlock_fair () ; } defer ! (s . mutex . raw . lock ()) ; f () } # [doc = " Temporarily yields the mutex to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_fair` followed"] # [doc = " by `lock`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . mutex . raw . bump () ; } } }
    };
}

impl_24!()