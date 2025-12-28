macro_rules! deps {
    () => {
        ReentrantMutexGuard!();
        RawMutexFair!();
        GetThreadId!();
        ReentrantMutex!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < R : RawMutexFair , G : GetThreadId , T : ? Sized > ReentrantMutex < R , G , T > { # [doc = " Forcibly unlocks the mutex using a fair unlock protocol."] # [doc = ""] # [doc = " This is useful when combined with `mem::forget` to hold a lock without"] # [doc = " the need to maintain a `ReentrantMutexGuard` object alive, for example when"] # [doc = " dealing with FFI."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method must only be called if the current thread logically owns a"] # [doc = " `ReentrantMutexGuard` but that guard has be discarded using `mem::forget`."] # [doc = " Behavior is undefined if a mutex is unlocked when not locked."] # [inline] # [track_caller] pub unsafe fn force_unlock_fair (& self) { self . raw . unlock_fair () ; } }
    };
}

impl_64!();