macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! MutexGuard {
    () => {
        deps!();
        # [doc = " An RAII implementation of a \"scoped lock\" of a mutex. When this structure is"] # [doc = " dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " `Deref` and `DerefMut` implementations."] # [clippy :: has_significant_drop] # [must_use = "if unused the Mutex will immediately unlock"] pub struct MutexGuard < 'a , R : RawMutex , T : ? Sized > { mutex : & 'a Mutex < R , T > , marker : PhantomData < (& 'a mut T , R :: GuardMarker) > , }
    };
}

MutexGuard!()