macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! OwnedMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII guard returned by the `lock_owned` and `try_lock_owned` methods."] # [doc = " When this structure is dropped (falls out of scope), the lock will be"] # [doc = " unlocked."] pub struct OwnedMutexGuard < T : ? Sized > { mutex : Arc < Mutex < T > > , }
    };
}

OwnedMutexGuard!()