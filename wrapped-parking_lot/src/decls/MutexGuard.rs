macro_rules! deps {
    () => {
        RawMutex!();
    };
}

macro_rules! MutexGuard {
    () => {
        deps!();
        # [doc = " An RAII implementation of a \"scoped lock\" of a mutex. When this structure is"] # [doc = " dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " `Deref` and `DerefMut` implementations."] pub type MutexGuard < 'a , T > = lock_api :: MutexGuard < 'a , RawMutex , T > ;
    };
}

MutexGuard!()