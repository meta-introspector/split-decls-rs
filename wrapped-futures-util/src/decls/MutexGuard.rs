macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! MutexGuard {
    () => {
        deps!();
        # [doc = " An RAII guard returned by the `lock` and `try_lock` methods."] # [doc = " When this structure is dropped (falls out of scope), the lock will be"] # [doc = " unlocked."] pub struct MutexGuard < 'a , T : ? Sized > { mutex : & 'a Mutex < T > , }
    };
}

MutexGuard!()