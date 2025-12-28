macro_rules! deps {
    () => {
        GetThreadId!();
        ReentrantMutex!();
        RawReentrantMutex!();
        RawMutex!();
        ReentrantMutexGuard!();
    };
}

macro_rules! MappedReentrantMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedReentrantMutexGuard` and `ReentrantMutexGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] # [clippy :: has_significant_drop] # [must_use = "if unused the ReentrantMutex will immediately unlock"] pub struct MappedReentrantMutexGuard < 'a , R : RawMutex , G : GetThreadId , T : ? Sized > { raw : & 'a RawReentrantMutex < R , G > , data : * const T , marker : PhantomData < & 'a T > , }
    };
}

MappedReentrantMutexGuard!();