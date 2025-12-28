macro_rules! deps {
    () => {
        GetThreadId!();
        ReentrantMutex!();
        GuardNoSend!();
        RawMutex!();
    };
}

macro_rules! ReentrantMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII implementation of a \"scoped lock\" of a reentrant mutex. When this structure"] # [doc = " is dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " `Deref` implementation."] # [clippy :: has_significant_drop] # [must_use = "if unused the ReentrantMutex will immediately unlock"] pub struct ReentrantMutexGuard < 'a , R : RawMutex , G : GetThreadId , T : ? Sized > { remutex : & 'a ReentrantMutex < R , G , T > , marker : PhantomData < (& 'a T , GuardNoSend) > , }
    };
}

ReentrantMutexGuard!();